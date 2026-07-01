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

//! Severity tallying for commands that run an ordered list of checks,
//! such as `lint` or `verify`, and must decide between full and
//! partial success.

use std::fmt;

/// The severity of one finding reported by a check.
///
/// Variants are declared in ascending order of severity, so `Ord`
/// comparisons and `max` select the more severe value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Severity {
    /// Informational only; never affects the result.
    Info,
    /// Degrades the result to partial success only under a strict
    /// policy, such as a `--strict` flag.
    Warning,
    /// Always degrades the result to partial success.
    Error,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        })
    }
}

/// Counts [`Severity`] occurrences recorded while running an ordered
/// list of checks, and decides whether the result should be reported
/// as partial success.
///
/// `Tally` does not reference `Outcome`. It answers
/// [`is_partial_success_worthy`](Self::is_partial_success_worthy) with
/// a `bool`, and the caller maps that to `Outcome::Success` or
/// `Outcome::PartialSuccess`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Tally {
    info: u32,
    warning: u32,
    error: u32,
}

impl Tally {
    /// A tally with no findings recorded.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records one occurrence of `severity`.
    pub fn record(&mut self, severity: Severity) {
        match severity {
            Severity::Info => self.info += 1,
            Severity::Warning => self.warning += 1,
            Severity::Error => self.error += 1,
        }
    }

    /// Number of `Info`-severity findings recorded.
    #[must_use]
    pub fn info(&self) -> u32 {
        self.info
    }

    /// Number of `Warning`-severity findings recorded.
    #[must_use]
    pub fn warning(&self) -> u32 {
        self.warning
    }

    /// Number of `Error`-severity findings recorded.
    #[must_use]
    pub fn error(&self) -> u32 {
        self.error
    }

    /// Total findings recorded, all severities combined.
    #[must_use]
    pub fn total(&self) -> u32 {
        self.info + self.warning + self.error
    }

    /// `true` if no `Warning` or `Error` was recorded. `Info` findings
    /// do not affect this.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.warning == 0 && self.error == 0
    }

    /// The most severe [`Severity`] recorded, or `None` if nothing was
    /// recorded yet.
    #[must_use]
    pub fn worst_severity(&self) -> Option<Severity> {
        [
            (Severity::Info, self.info),
            (Severity::Warning, self.warning),
            (Severity::Error, self.error),
        ]
        .into_iter()
        .filter(|(_, count)| *count > 0)
        .map(|(severity, _)| severity)
        .max()
    }

    /// Whether this tally should be reported as partial success.
    ///
    /// Any `Error` counts. A `Warning` counts only when `strict` is
    /// `true`.
    #[must_use]
    pub fn is_partial_success_worthy(&self, strict: bool) -> bool {
        self.error > 0 || (strict && self.warning > 0)
    }
}

impl fmt::Display for Tally {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} info, {} warning, {} error",
            self.info, self.warning, self.error
        )
    }
}
