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
    DEFAULT_TERMINAL_WIDTH, Padding, StyledTable, TableStyle, build_table_with_columns,
    display_option, display_option_or, parse_columns, responsive_width, terminal_width,
};
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

#[test]
fn test_styled_table_remove_header_row() {
    let data = vec![
        TestRow {
            name: "alice".into(),
            value: 1,
        },
        TestRow {
            name: "bob".into(),
            value: 2,
        },
    ];

    let table = StyledTable::new().remove_header_row().build(data);
    let output = table.to_string();
    assert!(!output.contains("name"));
    assert!(!output.contains("value"));
    assert!(output.contains("alice"));
    assert!(output.contains("bob"));
}

#[test]
fn test_styled_table_custom_padding() {
    let data = vec![TestRow {
        name: "test".into(),
        value: 42,
    }];

    let table = StyledTable::new()
        .padding(Padding::new(0, 1, 0, 0))
        .build(data);
    let output = table.to_string();
    assert!(output.contains("test"));
}

#[derive(Tabled, Clone)]
struct MultiLineRow {
    name: String,
    tags: String,
}

#[test]
fn test_styled_table_replace_newlines() {
    let data = vec![MultiLineRow {
        name: "item".into(),
        tags: "tag1\ntag2\ntag3".into(),
    }];

    let table = StyledTable::new().replace_newlines(",").build(data);
    let output = table.to_string();
    assert!(output.contains("tag1,tag2,tag3"));
    assert!(!output.contains("tag1\n"));
}

#[test]
fn test_styled_table_borderless_with_all_options() {
    let data = vec![
        MultiLineRow {
            name: "item1".into(),
            tags: "a\nb".into(),
        },
        MultiLineRow {
            name: "item2".into(),
            tags: "c\nd".into(),
        },
    ];

    let table = StyledTable::new()
        .style(TableStyle::Borderless)
        .remove_header_row()
        .padding(Padding::new(0, 1, 0, 0))
        .replace_newlines(",")
        .build(data);
    let output = table.to_string();
    assert!(output.contains("item1"));
    assert!(output.contains("a,b"));
    assert!(!output.contains("name"));
}

#[test]
fn test_styled_table_header_with_remove_header_row() {
    let data = vec![
        TestRow {
            name: "alice".into(),
            value: 1,
        },
        TestRow {
            name: "bob".into(),
            value: 2,
        },
    ];

    let table = StyledTable::new()
        .header("My Table")
        .remove_header_row()
        .build(data);
    let output = table.to_string();
    assert!(output.contains("My Table"));
    assert!(!output.contains("name"));
    assert!(!output.contains("value"));
    assert!(output.contains("alice"));
    assert!(output.contains("bob"));
}

#[test]
fn test_parse_columns_basic() {
    let cols = parse_columns("name,value");
    assert_eq!(cols, vec!["name", "value"]);
}

#[test]
fn test_parse_columns_with_whitespace() {
    let cols = parse_columns("name , value , status");
    assert_eq!(cols, vec!["name", "value", "status"]);
}

#[test]
fn test_parse_columns_case_insensitive() {
    let cols = parse_columns("Name,VALUE,Status");
    assert_eq!(cols, vec!["name", "value", "status"]);
}

#[test]
fn test_parse_columns_empty_entries_filtered() {
    let cols = parse_columns("name,,value,");
    assert_eq!(cols, vec!["name", "value"]);
}

#[test]
fn test_parse_columns_empty_string() {
    let cols = parse_columns("");
    assert!(cols.is_empty());
}

#[test]
fn test_parse_columns_whitespace_only() {
    let cols = parse_columns("  ,  ,  ");
    assert!(cols.is_empty());
}

#[test]
fn test_build_table_with_columns_selects_correct_columns() {
    let data = vec![
        TestRow {
            name: "alice".into(),
            value: 42,
        },
        TestRow {
            name: "bob".into(),
            value: 99,
        },
    ];

    let columns = parse_columns("name");
    let table = build_table_with_columns(&data, &columns);
    let output = table.to_string();

    assert!(output.contains("name"));
    assert!(output.contains("alice"));
    assert!(output.contains("bob"));
    assert!(!output.contains("42"));
    assert!(!output.contains("99"));
}

#[test]
fn test_build_table_with_columns_multiple() {
    let data = vec![TestRow {
        name: "test".into(),
        value: 123,
    }];

    let columns = parse_columns("value,name");
    let table = build_table_with_columns(&data, &columns);
    let output = table.to_string();

    assert!(output.contains("value"));
    assert!(output.contains("name"));
    assert!(output.contains("test"));
    assert!(output.contains("123"));
}

#[test]
fn test_build_table_with_columns_unknown_ignored() {
    let data = vec![TestRow {
        name: "test".into(),
        value: 1,
    }];

    let columns = parse_columns("name,unknown,nonexistent");
    let table = build_table_with_columns(&data, &columns);
    let output = table.to_string();

    assert!(output.contains("name"));
    assert!(output.contains("test"));
    assert!(!output.contains("unknown"));
    assert!(!output.contains("nonexistent"));
}

#[test]
fn test_build_table_with_columns_empty_data() {
    let data: Vec<TestRow> = vec![];
    let columns = parse_columns("name,value");
    let table = build_table_with_columns(&data, &columns);
    let output = table.to_string();

    assert!(output.contains("name"));
    assert!(output.contains("value"));
}

#[test]
fn test_build_table_with_columns_no_valid_columns() {
    let data = vec![TestRow {
        name: "test".into(),
        value: 1,
    }];

    let columns = parse_columns("unknown,missing");
    let table = build_table_with_columns(&data, &columns);
    let output = table.to_string();

    assert!(!output.contains("test"));
    assert!(!output.contains("name"));
}

#[derive(Tabled, Clone)]
struct ThreeColumnRow {
    id: u32,
    name: String,
    status: String,
}

#[test]
fn test_build_table_with_columns_preserves_order() {
    let data = vec![ThreeColumnRow {
        id: 1,
        name: "first".into(),
        status: "active".into(),
    }];

    let columns = parse_columns("status,id");
    let table = build_table_with_columns(&data, &columns);
    let output = table.to_string();

    let status_pos = output.find("status").unwrap();
    let id_pos = output.find("id").unwrap();
    assert!(status_pos < id_pos);
}

#[test]
fn test_build_table_with_columns_duplicate_columns() {
    let data = vec![TestRow {
        name: "test".into(),
        value: 42,
    }];

    let columns = parse_columns("name,name");
    let table = build_table_with_columns(&data, &columns);
    let output = table.to_string();

    assert!(output.contains("name"));
    assert!(output.contains("test"));
}

#[test]
fn test_terminal_width_returns_positive() {
    let width = terminal_width();
    assert!(width > 0);
}

#[test]
fn test_default_terminal_width_constant() {
    assert_eq!(DEFAULT_TERMINAL_WIDTH, 120);
}

#[test]
fn test_responsive_width_full() {
    let width = responsive_width(1.0);
    assert!(width > 0);
}

#[test]
fn test_responsive_width_half() {
    let full = terminal_width();
    let half = responsive_width(0.5);
    assert!(half <= full);
}

#[test]
fn test_responsive_width_clamps_over_one() {
    let full = terminal_width();
    let over = responsive_width(1.5);
    assert_eq!(over, full);
}

#[test]
fn test_responsive_width_clamps_negative() {
    let zero = responsive_width(-0.5);
    assert_eq!(zero, 0);
}

#[test]
fn test_styled_table_max_width() {
    let data = vec![TestRow {
        name: "test".into(),
        value: 42,
    }];

    let table = StyledTable::new().max_width(100).build(data);
    let _ = table.to_string();
}

#[test]
fn test_styled_table_wrap_column() {
    let data = vec![MultiLineRow {
        name: "item".into(),
        tags: "very long content that should be wrapped at some point".into(),
    }];

    let table = StyledTable::new().wrap_column(1, 20).build(data);
    let _ = table.to_string();
}

#[test]
fn test_styled_table_responsive_combo() {
    let data = vec![TestRow {
        name: "test".into(),
        value: 123,
    }];

    let table = StyledTable::new()
        .style(TableStyle::Modern)
        .wrap_column(0, 30)
        .max_width(responsive_width(0.8))
        .build(data);
    let _ = table.to_string();
}

mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn parse_columns_always_lowercase(input in "[a-zA-Z,\\s]{0,50}") {
            let cols = parse_columns(&input);
            for col in &cols {
                assert_eq!(col, &col.to_lowercase());
            }
        }

        #[test]
        fn parse_columns_no_empty_entries(input in "[a-zA-Z,\\s]{0,50}") {
            let cols = parse_columns(&input);
            for col in &cols {
                assert!(!col.is_empty());
            }
        }

        #[test]
        fn parse_columns_no_whitespace(input in "[a-zA-Z,\\s]{0,50}") {
            let cols = parse_columns(&input);
            for col in &cols {
                assert_eq!(col, col.trim());
            }
        }
    }
}
