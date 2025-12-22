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
