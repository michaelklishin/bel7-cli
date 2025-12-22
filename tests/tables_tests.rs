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

use bel7_cli::{StyledTable, TableStyle, display_option, display_option_or};
use tabled::Tabled;

#[derive(Tabled, Clone)]
struct TestRow {
    name: String,
    value: i32,
}

#[test]
fn test_styled_table_builds() {
    let data = vec![
        TestRow {
            name: "a".into(),
            value: 1,
        },
        TestRow {
            name: "b".into(),
            value: 2,
        },
    ];

    let table = StyledTable::new()
        .style(TableStyle::Modern)
        .header("Test Table")
        .build(data);

    let output = table.to_string();
    assert!(output.contains("name"));
    assert!(output.contains("Test Table"));
}

#[test]
fn test_display_option() {
    assert_eq!(display_option(&Some(42)), "42");
    assert_eq!(display_option::<i32>(&None), "");
}

#[test]
fn test_display_option_or() {
    assert_eq!(display_option_or(&Some(42), "N/A"), "42");
    assert_eq!(display_option_or::<i32>(&None, "N/A"), "N/A");
}

#[test]
fn test_table_style_default() {
    assert_eq!(TableStyle::default(), TableStyle::Modern);
}

#[test]
fn test_all_table_styles_apply() {
    let styles = [
        TableStyle::Modern,
        TableStyle::Borderless,
        TableStyle::Markdown,
        TableStyle::Sharp,
        TableStyle::Ascii,
        TableStyle::Psql,
        TableStyle::Dots,
    ];

    let data = vec![TestRow {
        name: "test".into(),
        value: 1,
    }];

    for style in styles {
        let table = StyledTable::new().style(style).build(data.clone());
        let output = table.to_string();
        assert!(output.contains("test"));
    }
}

#[test]
fn test_styled_table_without_header() {
    let data = vec![TestRow {
        name: "a".into(),
        value: 1,
    }];

    let table = StyledTable::new().build(data);
    let output = table.to_string();
    assert!(output.contains("name"));
    assert!(output.contains("a"));
}

#[test]
fn test_styled_table_empty_data() {
    let data: Vec<TestRow> = vec![];
    let table = StyledTable::new().header("Empty").build(data);
    let output = table.to_string();
    assert!(output.contains("Empty"));
}
