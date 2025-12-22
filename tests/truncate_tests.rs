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

use bel7_cli::{truncate_middle, truncate_string, truncate_with_suffix};

#[test]
fn test_truncate_short_string() {
    assert_eq!(truncate_string("Hi", 10), "Hi");
}

#[test]
fn test_truncate_exact_length() {
    assert_eq!(truncate_string("Hello", 5), "Hello");
}

#[test]
fn test_truncate_long_string() {
    assert_eq!(truncate_string("Hello, World!", 8), "Hello...");
}

#[test]
fn test_truncate_unicode() {
    assert_eq!(truncate_string("Héllo Wörld", 8), "Héllo...");
}

#[test]
fn test_truncate_middle_short() {
    assert_eq!(truncate_middle("short", 20), "short");
}

#[test]
fn test_truncate_middle_long() {
    let result = truncate_middle("/very/long/path/to/file.txt", 20);
    assert!(result.len() <= 20);
    assert!(result.contains("..."));
}

#[test]
fn test_custom_suffix() {
    assert_eq!(truncate_with_suffix("Hello, World!", 9, "…"), "Hello, W…");
}

#[test]
fn test_truncate_empty_string() {
    assert_eq!(truncate_string("", 10), "");
    assert_eq!(truncate_middle("", 10), "");
}

#[test]
fn test_truncate_middle_max_chars_zero() {
    assert_eq!(truncate_middle("Hello", 0), "");
}

#[test]
fn test_truncate_middle_max_chars_smaller_than_suffix() {
    assert_eq!(truncate_middle("Hello, World!", 1), ".");
    assert_eq!(truncate_middle("Hello, World!", 2), "..");
    assert_eq!(truncate_middle("Hello, World!", 3), "...");
}

#[test]
fn test_truncate_middle_preserves_both_ends() {
    let result = truncate_middle("abcdefghij", 7);
    assert_eq!(result, "ab...ij");
}

#[test]
fn test_truncate_with_unicode_suffix() {
    assert_eq!(truncate_with_suffix("Hello, World!", 6, "…"), "Hello…");
}
