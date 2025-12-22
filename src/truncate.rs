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

//! String truncation utilities that can be used by [`std::fmt::Display`] implementations.

/// Default suffix appended to truncated strings.
pub const DEFAULT_TRUNCATION_SUFFIX: &str = "...";

/// Truncates a string to a maximum number of characters.
///
/// If the string exceeds `max_chars`, it's truncated and the suffix
/// (default "...") is appended. The total length including suffix
/// will not exceed `max_chars`.
///
/// # Example
///
/// ```
/// use bel7_cli::truncate_string;
///
/// assert_eq!(truncate_string("Hello", 10), "Hello");
/// assert_eq!(truncate_string("Hello, World!", 8), "Hello...");
/// ```
pub fn truncate_string(s: &str, max_chars: usize) -> String {
    truncate_with_suffix(s, max_chars, DEFAULT_TRUNCATION_SUFFIX)
}

/// Truncates a string with a custom suffix.
///
/// # Example
///
/// ```
/// use bel7_cli::truncate_with_suffix;
///
/// assert_eq!(truncate_with_suffix("Hello, World!", 8, "…"), "Hello, …");
/// ```
pub fn truncate_with_suffix(s: &str, max_chars: usize, suffix: &str) -> String {
    let char_count = s.chars().count();

    if char_count <= max_chars {
        return s.to_string();
    }

    let suffix_len = suffix.chars().count();
    let take_chars = max_chars.saturating_sub(suffix_len);

    let truncated: String = s.chars().take(take_chars).collect();
    format!("{}{}", truncated, suffix)
}

/// Truncates a string in the middle, keeping start and end.
///
/// Useful for file paths or long identifiers where both ends are important.
///
/// # Example
///
/// ```
/// use bel7_cli::truncate_middle;
///
/// let result = truncate_middle("/very/long/path/to/file.txt", 20);
/// assert!(result.len() <= 20);
/// assert!(result.contains("..."));
/// ```
pub fn truncate_middle(s: &str, max_chars: usize) -> String {
    let char_count = s.chars().count();

    if char_count <= max_chars {
        return s.to_string();
    }

    let suffix = "...";
    let suffix_char_count = suffix.chars().count();

    if max_chars <= suffix_char_count {
        return suffix.chars().take(max_chars).collect();
    }

    let available = max_chars - suffix_char_count;
    let start_len = available.div_ceil(2);
    let end_len = available / 2;

    let start: String = s.chars().take(start_len).collect();
    let end: String = s.chars().skip(char_count - end_len).collect();

    format!("{}{}{}", start, suffix, end)
}
