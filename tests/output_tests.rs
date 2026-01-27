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

use bel7_cli::{
    format_bold, format_dimmed, format_error, format_info, format_success, format_warning,
    should_colorize, should_colorize_stderr,
};

#[test]
fn test_format_functions_dont_panic() {
    let _ = format_success("ok");
    let _ = format_error("err");
    let _ = format_warning("warn");
    let _ = format_info("info");
    let _ = format_dimmed("dim");
    let _ = format_bold("bold");
}

#[test]
fn test_should_colorize_returns_bool() {
    let result = should_colorize();
    let _ = result;
}

#[test]
fn test_should_colorize_stderr_returns_bool() {
    let result = should_colorize_stderr();
    let _ = result;
}

#[test]
fn test_format_success_returns_string() {
    let result = format_success("test");
    assert!(result.contains("test"));
}

#[test]
fn test_format_error_returns_string() {
    let result = format_error("test");
    assert!(result.contains("test"));
}

#[test]
fn test_format_warning_returns_string() {
    let result = format_warning("test");
    assert!(result.contains("test"));
}

#[test]
fn test_format_info_returns_string() {
    let result = format_info("test");
    assert!(result.contains("test"));
}

#[test]
fn test_format_dimmed_returns_string() {
    let result = format_dimmed("test");
    assert!(result.contains("test"));
}

#[test]
fn test_format_bold_returns_string() {
    let result = format_bold("test");
    assert!(result.contains("test"));
}
