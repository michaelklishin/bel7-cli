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

use owo_colors::OwoColorize;
use std::fmt::Display;

/// Prints a success message with a green checkmark prefix.
///
/// # Example
///
/// ```
/// use bel7_cli::print_success;
///
/// print_success("Operation completed");
/// // Output: ✓ Operation completed (green checkmark)
/// ```
pub fn print_success(message: impl Display) {
    println!("{} {}", "✓".green().bold(), message);
}

/// Prints an error message to stderr with a red X prefix.
///
/// # Example
///
/// ```
/// use bel7_cli::print_error;
///
/// print_error("Something went wrong");
/// // Output: ✗ Something went wrong (red X)
/// ```
pub fn print_error(message: impl Display) {
    eprintln!("{} {}", "✗".red().bold(), message);
}

/// Prints a warning message with a yellow exclamation prefix.
///
/// # Example
///
/// ```
/// use bel7_cli::print_warning;
///
/// print_warning("This might cause issues");
/// // Output: ! This might cause issues (yellow !)
/// ```
pub fn print_warning(message: impl Display) {
    println!("{} {}", "!".yellow().bold(), message);
}

/// Prints an info message with a blue arrow prefix.
///
/// # Example
///
/// ```
/// use bel7_cli::print_info;
///
/// print_info("Processing files...");
/// // Output: → Processing files... (blue arrow)
/// ```
pub fn print_info(message: impl Display) {
    println!("{} {}", "→".blue().bold(), message);
}

/// Prints a dimmed/muted message.
///
/// Useful for secondary information or hints.
pub fn print_dimmed(message: impl Display) {
    println!("{}", message.to_string().dimmed());
}

/// Formats a value as success (green).
pub fn format_success<T: Display>(value: T) -> String {
    format!("{}", value.green())
}

/// Formats a value as error (red).
pub fn format_error<T: Display>(value: T) -> String {
    format!("{}", value.red())
}

/// Formats a value as warning (yellow).
pub fn format_warning<T: Display>(value: T) -> String {
    format!("{}", value.yellow())
}

/// Formats a value as info (blue).
pub fn format_info<T: Display>(value: T) -> String {
    format!("{}", value.blue())
}

/// Formats a value as dimmed/muted.
pub fn format_dimmed<T: Display>(value: T) -> String {
    format!("{}", value.dimmed())
}

/// Formats a value as bold.
pub fn format_bold<T: Display>(value: T) -> String {
    format!("{}", value.bold())
}
