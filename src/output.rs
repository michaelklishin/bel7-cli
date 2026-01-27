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

//! Colored console output utilities.
//!
//! Provides consistent, colored output for CLI applications.
//! Respects the `NO_COLOR` environment variable and detects non-TTY output.

use std::env;
use std::fmt::Display;
use std::io::IsTerminal;

use owo_colors::OwoColorize;

/// Returns whether colored output should be used.
///
/// Returns `false` if the `NO_COLOR` environment variable is set (any value)
/// or `stdout` is not a terminal (that is, piped or redirected).
///
/// This follows the [NO_COLOR standard](https://no-color.org/).
#[must_use]
pub fn should_colorize() -> bool {
    env::var("NO_COLOR").is_err() && std::io::stdout().is_terminal()
}

/// Returns whether colored output should be used for stderr.
///
/// Returns `false` if the `NO_COLOR` environment variable is set (any value)
/// or `stdout` is not a terminal (that is, piped or redirected).
#[must_use]
pub fn should_colorize_stderr() -> bool {
    env::var("NO_COLOR").is_err() && std::io::stderr().is_terminal()
}

/// Prints a success message with a green checkmark prefix.
///
/// Respects `NO_COLOR` and terminal detection.
pub fn print_success(message: impl Display) {
    if should_colorize() {
        println!("{} {}", "✓".green().bold(), message);
    } else {
        println!("✓ {}", message);
    }
}

/// Prints an error message to stderr with a red X prefix.
///
/// Respects `NO_COLOR` and terminal detection.
pub fn print_error(message: impl Display) {
    if should_colorize_stderr() {
        eprintln!("{} {}", "✗".red().bold(), message);
    } else {
        eprintln!("✗ {}", message);
    }
}

/// Prints a warning message with a yellow exclamation prefix.
///
/// Respects `NO_COLOR` and terminal detection.
pub fn print_warning(message: impl Display) {
    if should_colorize() {
        println!("{} {}", "!".yellow().bold(), message);
    } else {
        println!("! {}", message);
    }
}

/// Prints an info message with a blue arrow prefix.
///
/// Respects `NO_COLOR` and terminal detection.
pub fn print_info(message: impl Display) {
    if should_colorize() {
        println!("{} {}", "→".blue().bold(), message);
    } else {
        println!("→ {}", message);
    }
}

/// Prints a dimmed/muted message.
///
/// Respects `NO_COLOR` and terminal detection.
pub fn print_dimmed(message: impl Display) {
    if should_colorize() {
        println!("{}", message.to_string().dimmed());
    } else {
        println!("{}", message);
    }
}

/// Formats a value as success (green) if colors are enabled.
#[must_use]
pub fn format_success<T: Display>(value: T) -> String {
    if should_colorize() {
        format!("{}", value.green())
    } else {
        value.to_string()
    }
}

/// Formats a value as error (red) if colors are enabled.
#[must_use]
pub fn format_error<T: Display>(value: T) -> String {
    if should_colorize() {
        format!("{}", value.red())
    } else {
        value.to_string()
    }
}

/// Formats a value as warning (yellow) if colors are enabled.
#[must_use]
pub fn format_warning<T: Display>(value: T) -> String {
    if should_colorize() {
        format!("{}", value.yellow())
    } else {
        value.to_string()
    }
}

/// Formats a value as info (blue) if colors are enabled.
#[must_use]
pub fn format_info<T: Display>(value: T) -> String {
    if should_colorize() {
        format!("{}", value.blue())
    } else {
        value.to_string()
    }
}

/// Formats a value as dimmed/muted if colors are enabled.
#[must_use]
pub fn format_dimmed<T: Display>(value: T) -> String {
    if should_colorize() {
        format!("{}", value.dimmed())
    } else {
        value.to_string()
    }
}

/// Formats a value as bold if colors are enabled.
#[must_use]
pub fn format_bold<T: Display>(value: T) -> String {
    if should_colorize() {
        format!("{}", value.bold())
    } else {
        value.to_string()
    }
}
