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

/// Common exit code mappings for typical error categories.
///
/// Use these as reference when implementing `ExitCodeProvider`.
pub mod codes {
    use sysexits::ExitCode;

    /// Exit code for successful completion.
    pub const OK: ExitCode = ExitCode::Ok;

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
