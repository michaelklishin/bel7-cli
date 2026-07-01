// Copyright (C) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! CLI exit code utilities.
//!
//! Provides exit code mapping following BSD sysexits conventions.

use std::error::Error;
use std::fmt;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::process;

pub use sysexits::ExitCode;

/// Extension trait for converting `ExitCode` to `i32`.
///
/// sysexits 0.11 removed direct integer conversions;
/// this trait provides the `to_i32` method for use with `process::exit`.
pub trait ExitCodeExt {
    fn to_i32(self) -> i32;
}

impl ExitCodeExt for ExitCode {
    fn to_i32(self) -> i32 {
        i32::from(u8::from(self))
    }
}

/// Trait for errors that can be mapped to CLI exit codes.
///
/// Uses BSD sysexits conventions for consistent exit code semantics.
///
/// # Example
///
/// ```
/// use bel7_cli::{ExitCodeProvider, ExitCode};
/// use thiserror::Error;
///
/// #[derive(Error, Debug)]
/// enum MyError {
///     #[error("file not found: {0}")]
///     FileNotFound(String),
///     #[error("permission denied")]
///     PermissionDenied,
///     #[error("invalid input")]
///     InvalidInput,
/// }
///
/// impl ExitCodeProvider for MyError {
///     fn exit_code(&self) -> ExitCode {
///         match self {
///             MyError::FileNotFound(_) => ExitCode::NoInput,
///             MyError::PermissionDenied => ExitCode::NoPerm,
///             MyError::InvalidInput => ExitCode::DataErr,
///         }
///     }
/// }
/// ```
pub trait ExitCodeProvider: Error {
    /// Returns the appropriate exit code for this error.
    fn exit_code(&self) -> ExitCode;
}

/// Exit code for the partial-success outcome: the program ran successfully
/// but not every unit of work completed.
///
/// Use for verbs that operate over a set (commit list, file batch, test
/// sweep) where the caller asked for "as much as possible" and the runtime
/// delivered some-but-not-all. Distinct from `ExitCode::Ok` (everything
/// succeeded) and from the `sysexits` error families (something went
/// wrong). A shell consumer that wants "did anything work?" branches on
/// `code == 0 || code == PARTIAL_SUCCESS_I32`.
///
/// `sysexits` has no slot for partial success — its codes are binary
/// success-or-error. The number sits in the small-integer band alongside
/// conventions like `diff`'s exit 1 ("differences found") and `rsync`'s
/// exit 23 ("partial transfer"), without colliding with sysexits'
/// `EX_USAGE = 64` and above.
///
/// Mirrored as [`PARTIAL_SUCCESS_U8`].
pub const PARTIAL_SUCCESS_I32: i32 = 3;

/// `u8` mirror of [`PARTIAL_SUCCESS_I32`] for consumers piping through
/// [`process::ExitCode::from(u8)`](process::ExitCode). The u8-fit is
/// enforced at compile time by the assertion below.
pub const PARTIAL_SUCCESS_U8: u8 = 3;

const _: () = assert!(
    PARTIAL_SUCCESS_I32 >= 0
        && PARTIAL_SUCCESS_I32 <= u8::MAX as i32
        && PARTIAL_SUCCESS_I32 as u8 == PARTIAL_SUCCESS_U8,
    "PARTIAL_SUCCESS_I32 must fit in u8 and agree with PARTIAL_SUCCESS_U8",
);

/// Common exit code mappings for typical error categories.
///
/// Use these as reference when implementing `ExitCodeProvider`.
pub mod codes {
    use sysexits::ExitCode;

    /// Exit code for successful completion.
    pub const OK: ExitCode = ExitCode::Ok;

    /// Exit code for partial success: ran successfully but not every
    /// unit of work completed. Re-exported as a plain `i32` because
    /// `sysexits::ExitCode` is sealed upstream and has no
    /// `PartialSuccess` variant. The canonical definition is
    /// [`super::PARTIAL_SUCCESS_I32`].
    pub const PARTIAL_SUCCESS: i32 = super::PARTIAL_SUCCESS_I32;

    /// `u8` mirror of [`PARTIAL_SUCCESS`]; canonical at
    /// [`super::PARTIAL_SUCCESS_U8`].
    pub const PARTIAL_SUCCESS_U8: u8 = super::PARTIAL_SUCCESS_U8;

    /// Exit code for general errors.
    pub const SOFTWARE: ExitCode = ExitCode::Software;

    /// Exit code for I/O errors.
    pub const IO_ERR: ExitCode = ExitCode::IoErr;

    /// Exit code for missing input files.
    pub const NO_INPUT: ExitCode = ExitCode::NoInput;

    /// Exit code for permission errors.
    pub const NO_PERM: ExitCode = ExitCode::NoPerm;

    /// Exit code for invalid user data.
    pub const DATA_ERR: ExitCode = ExitCode::DataErr;

    /// Exit code for configuration errors.
    pub const CONFIG: ExitCode = ExitCode::Config;

    /// Exit code for OS-level errors.
    pub const OS_ERR: ExitCode = ExitCode::OsErr;

    /// Exit code for unavailable services.
    pub const UNAVAILABLE: ExitCode = ExitCode::Unavailable;

    /// Exit code for temporary failures (retry may succeed).
    pub const TEMP_FAIL: ExitCode = ExitCode::TempFail;

    /// Exit code for protocol errors.
    pub const PROTOCOL: ExitCode = ExitCode::Protocol;

    /// Exit code for usage/command-line syntax errors.
    pub const USAGE: ExitCode = ExitCode::Usage;
}

/// A three-way outcome for command handlers that distinguish full
/// success, partial success, and failure.
///
/// `ExitCodeProvider` covers the binary success-or-error world via
/// [`run_with_exit_code`]. `Outcome` extends it with a third arm so a
/// verb that operates over a set can report "some-but-not-all" without
/// abusing an error variant.
#[derive(Debug)]
pub enum Outcome<E>
where
    E: ExitCodeProvider,
{
    /// Every unit of work completed.
    Success,
    /// Some units completed, others did not. Maps to
    /// [`PARTIAL_SUCCESS_I32`].
    PartialSuccess,
    /// Something went wrong. The error's `ExitCodeProvider` impl
    /// supplies the exit code.
    ///
    /// `ExitCodeProvider` implementations **must not** return
    /// `ExitCode::Ok` — doing so collapses the failure to exit `0`
    /// in [`Outcome::exit_code_i32`] and silently misclassifies it
    /// as success. The trait cannot enforce this at the type level.
    Failure(E),
}

impl<E> Outcome<E>
where
    E: ExitCodeProvider,
{
    /// `PartialSuccess` if `degraded` is `true`, `Success` otherwise.
    ///
    /// Collapses the `if degraded { PartialSuccess } else { Success }`
    /// mapping at the end of a command handler. Composes with
    /// [`Tally::is_partial_success_worthy`](crate::Tally::is_partial_success_worthy).
    #[must_use]
    pub fn partial_if(degraded: bool) -> Self {
        if degraded {
            Outcome::PartialSuccess
        } else {
            Outcome::Success
        }
    }

    /// The integer exit code this outcome corresponds to.
    ///
    /// Borrows rather than consuming so the caller can log the error
    /// after computing the code.
    pub fn exit_code_i32(&self) -> i32 {
        match self {
            Outcome::Success => ExitCode::Ok.to_i32(),
            Outcome::PartialSuccess => PARTIAL_SUCCESS_I32,
            Outcome::Failure(e) => e.exit_code().to_i32(),
        }
    }

    /// Convert into the `Result<u8, E>` shape that consumers using
    /// `process::ExitCode::from(u8)` need.
    ///
    /// `Success` and `PartialSuccess` collapse to their `Ok(u8)` codes;
    /// `Failure(e)` propagates the error unchanged so the caller can
    /// log a `caused by:` chain before exiting.
    pub fn into_result_u8(self) -> Result<u8, E> {
        match self {
            Outcome::Success => Ok(0),
            Outcome::PartialSuccess => Ok(PARTIAL_SUCCESS_U8),
            Outcome::Failure(e) => Err(e),
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self, Outcome::Success)
    }

    pub fn is_partial_success(&self) -> bool {
        matches!(self, Outcome::PartialSuccess)
    }

    pub fn is_failure(&self) -> bool {
        matches!(self, Outcome::Failure(_))
    }
}

impl<E> fmt::Display for Outcome<E>
where
    E: ExitCodeProvider + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Outcome::Success => write!(f, "success (exit 0)"),
            Outcome::PartialSuccess => {
                write!(f, "partial success (exit {PARTIAL_SUCCESS_I32})")
            }
            Outcome::Failure(e) => {
                write!(f, "failure (exit {}): {e}", e.exit_code().to_i32())
            }
        }
    }
}

/// Helper function to run a main function and exit with appropriate code.
///
/// # Example
///
/// ```ignore
/// fn main() {
///     bel7_cli::run_with_exit_code(real_main);
/// }
///
/// fn real_main() -> Result<(), MyError> {
///     // ... your logic
///     Ok(())
/// }
/// ```
pub fn run_with_exit_code<E, F>(f: F) -> !
where
    E: ExitCodeProvider,
    F: FnOnce() -> Result<(), E>,
{
    match f() {
        Ok(()) => process::exit(ExitCode::Ok.to_i32()),
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(e.exit_code().to_i32())
        }
    }
}

/// Sibling of [`run_with_exit_code`] for handlers that distinguish
/// partial success.
///
/// On [`Outcome::PartialSuccess`] exits with [`PARTIAL_SUCCESS_I32`].
/// On [`Outcome::Failure`] prints the error to stderr and exits with
/// its `ExitCodeProvider` code. On [`Outcome::Success`] exits zero.
pub fn run_with_outcome<E, F>(f: F) -> !
where
    E: ExitCodeProvider,
    F: FnOnce() -> Outcome<E>,
{
    let outcome = f();
    if let Outcome::Failure(e) = &outcome {
        eprintln!("Error: {e}");
    }
    process::exit(outcome.exit_code_i32())
}

/// Consumer-side mirror of [`Outcome`] for Rust programs that read another
/// process's exit code (subprocess drivers, CI gates, batch wrappers).
///
/// [`Outcome`] is what a CLI verb *produces*; `ExitOutcome` is what a
/// downstream Rust caller *observes*. Both share the same constants
/// ([`PARTIAL_SUCCESS_I32`], `0`) so an ecosystem CLI that adopts
/// [`Outcome`] is automatically consumable through `ExitOutcome` without
/// per-consumer code.
///
/// Unlike [`Outcome`], the `Failure` arm carries the raw `i32` exit code
/// rather than a typed `E: ExitCodeProvider`, because the consumer
/// observes integers from another process — there is no typed error to
/// round-trip. Callers that need to branch on sysexits values (`64`
/// usage, `65` data, `75` temp-fail) read the integer directly out of
/// `Failure(i32)`.
///
/// Variants are declared in ascending order of severity, so `Ord`
/// comparisons and `max` select the worst outcome of a set. Two
/// `Failure` values compare by raw code, which carries no severity
/// meaning; the useful guarantee is that any `Failure` ranks above
/// `PartialSuccess`, which ranks above `Success`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExitOutcome {
    /// The subprocess exited `0`.
    Success,
    /// The subprocess exited [`PARTIAL_SUCCESS_I32`]. The caller should
    /// treat its stdout as a parseable payload (same as `Success`) but
    /// note that not every unit of work completed.
    PartialSuccess,
    /// The subprocess exited with any other code, or (on Unix) was
    /// killed by a signal. [`from_status`](Self::from_status) encodes
    /// Unix signal kills as `Failure(128 + signum)` — the shell
    /// convention — so `SIGTERM` (`143`) stays distinguishable from
    /// `SIGINT` (`130`) and from a literal `exit(143)`. A signal-less
    /// `None` status (rare) falls back to `Failure(-1)`.
    ///
    /// **Invariant.** Values constructed via
    /// [`from_code`](Self::from_code) or
    /// [`from_status`](Self::from_status) always carry an integer
    /// that is `!= 0 && != PARTIAL_SUCCESS_I32`. Manual construction
    /// (e.g. `Failure(0)`) bypasses the invariant; use
    /// [`canonicalize`](Self::canonicalize) to restore it.
    Failure(i32),
}

impl ExitOutcome {
    /// Classify a raw exit code. The primitive constructor —
    /// [`ExitOutcome::from_status`] and any other entry point reduce to
    /// this.
    pub fn from_code(code: i32) -> Self {
        match code {
            0 => Self::Success,
            c if c == PARTIAL_SUCCESS_I32 => Self::PartialSuccess,
            other => Self::Failure(other),
        }
    }

    /// Classify a [`process::ExitStatus`] from a child subprocess.
    ///
    /// On Unix, a signal kill (`status.code()` is `None`) maps to
    /// `Failure(128 + signum)` — the shell convention that keeps
    /// `SIGTERM` (143) distinguishable from `SIGINT` (130) and from a
    /// literal `exit(143)`. Unknown signal-less statuses fall back to
    /// `Failure(-1)`. On Windows, `status.code()` is always `Some`,
    /// so the helper just defers to [`Self::from_code`].
    pub fn from_status(status: &process::ExitStatus) -> Self {
        if let Some(code) = status.code() {
            return Self::from_code(code);
        }
        #[cfg(unix)]
        if let Some(signum) = ExitStatusExt::signal(status) {
            return Self::Failure(128 + signum);
        }
        Self::Failure(-1)
    }

    /// `true` for [`ExitOutcome::Success`] and
    /// [`ExitOutcome::PartialSuccess`] — the check a subprocess driver
    /// reaches for to decide "should I parse stdout as a payload?".
    pub fn is_success_shaped(&self) -> bool {
        matches!(self, Self::Success | Self::PartialSuccess)
    }

    /// `true` iff this is [`ExitOutcome::Success`].
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }

    /// `true` iff this is [`ExitOutcome::PartialSuccess`].
    pub fn is_partial_success(&self) -> bool {
        matches!(self, Self::PartialSuccess)
    }

    /// `true` iff this is [`ExitOutcome::Failure`].
    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Failure(_))
    }

    /// The exit code this outcome corresponds to. Round-trip:
    /// `ExitOutcome::from_code(n).code() == n` for every `i32`.
    pub fn code(&self) -> i32 {
        match self {
            Self::Success => 0,
            Self::PartialSuccess => PARTIAL_SUCCESS_I32,
            Self::Failure(c) => *c,
        }
    }

    /// Collapse a manually-constructed non-canonical state into its
    /// canonical arm. `Failure(0)` becomes `Success`,
    /// `Failure(PARTIAL_SUCCESS_I32)` becomes `PartialSuccess`,
    /// everything else is returned unchanged.
    ///
    /// Useful when an `ExitOutcome` is built from an integer field
    /// (e.g. a JSON envelope's `exit_code`) directly into the
    /// `Failure` variant; calling `canonicalize` afterwards restores
    /// the invariant [`from_code`](Self::from_code) maintains.
    pub fn canonicalize(self) -> Self {
        match self {
            Self::Failure(c) => Self::from_code(c),
            other => other,
        }
    }
}

impl fmt::Display for ExitOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success (exit 0)"),
            Self::PartialSuccess => {
                write!(f, "partial success (exit {PARTIAL_SUCCESS_I32})")
            }
            Self::Failure(c) => write!(f, "failure (exit {c})"),
        }
    }
}
