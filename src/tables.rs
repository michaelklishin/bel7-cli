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

//! Table styling utilities for CLI output.

use std::fmt::Display;
use tabled::{
    Table,
    settings::{
        Format, Modify, Panel, Remove,
        object::{Rows, Segment},
        style::Style,
    },
};

pub use tabled::settings::Padding;

/// Available table styles for CLI output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TableStyle {
    /// Modern rounded corners (default).
    #[default]
    Modern,
    /// No borders, space-separated.
    Borderless,
    /// Self-explanatory.
    Markdown,
    /// Sharp corners with box-drawing characters.
    Sharp,
    /// ASCII-only characters.
    Ascii,
    /// psql-style output.
    Psql,
    /// Uses dots for borders.
    Dots,
}

#[cfg(feature = "clap")]
impl clap::ValueEnum for TableStyle {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Modern,
            Self::Borderless,
            Self::Markdown,
            Self::Sharp,
            Self::Ascii,
            Self::Psql,
            Self::Dots,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(match self {
            Self::Modern => "modern",
            Self::Borderless => "borderless",
            Self::Markdown => "markdown",
            Self::Sharp => "sharp",
            Self::Ascii => "ascii",
            Self::Psql => "psql",
            Self::Dots => "dots",
        }))
    }
}

impl TableStyle {
    /// Applies this style to a table.
    pub fn apply(self, table: &mut Table) {
        match self {
            TableStyle::Modern => {
                table.with(Style::rounded());
            }
            TableStyle::Borderless => {
                table.with(Style::blank());
            }
            TableStyle::Markdown => {
                table.with(Style::markdown());
            }
            TableStyle::Sharp => {
                table.with(Style::sharp());
            }
            TableStyle::Ascii => {
                table.with(Style::ascii());
            }
            TableStyle::Psql => {
                table.with(Style::psql());
            }
            TableStyle::Dots => {
                table.with(Style::dots());
            }
        }
    }
}

/// A builder for styled tables.
pub struct StyledTable {
    style: TableStyle,
    header: Option<String>,
    remove_header_row: bool,
    padding: Option<Padding>,
    newline_replacement: Option<String>,
}

impl Default for StyledTable {
    fn default() -> Self {
        Self::new()
    }
}

impl StyledTable {
    /// Creates a new table builder with the default style.
    pub fn new() -> Self {
        Self {
            style: TableStyle::default(),
            header: None,
            remove_header_row: false,
            padding: None,
            newline_replacement: None,
        }
    }

    /// Sets the table style.
    pub fn style(mut self, style: TableStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets a header panel above the table.
    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Removes the first row (column headers) from the table.
    ///
    /// Useful for non-interactive/scriptable output where headers are noise.
    pub fn remove_header_row(mut self) -> Self {
        self.remove_header_row = true;
        self
    }

    /// Sets custom padding for table cells.
    ///
    /// Use `Padding::new(top, right, bottom, left)` to specify padding values.
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Replaces newlines in cell content with the specified string.
    ///
    /// Useful for non-interactive output where newlines would break parsing.
    /// Common replacement is `","` to turn multi-line values into comma-separated lists.
    pub fn replace_newlines(mut self, replacement: impl Into<String>) -> Self {
        self.newline_replacement = Some(replacement.into());
        self
    }

    /// Builds the final table from the provided data.
    pub fn build<T: tabled::Tabled>(self, data: Vec<T>) -> Table {
        let mut table = Table::new(data);

        self.style.apply(&mut table);

        if let Some(padding) = self.padding {
            table.with(padding);
        }

        // Remove column headers before adding panel header
        if self.remove_header_row {
            table.with(Remove::row(Rows::first()));
        }

        if let Some(header) = self.header {
            table.with(Panel::header(header));
        }

        if let Some(replacement) = self.newline_replacement {
            table.with(
                Modify::new(Segment::all())
                    .with(Format::content(move |s| s.replace('\n', &replacement))),
            );
        }

        table
    }
}

/// Formats an optional value for rendering in a table cell.
///
/// Returns an empty string for None, otherwise the Display representation.
pub fn display_option<T: Display>(opt: &Option<T>) -> String {
    opt.as_ref().map_or_else(String::new, |val| val.to_string())
}

/// Formats an optional value with a default.
pub fn display_option_or<T: Display>(opt: &Option<T>, default: &str) -> String {
    opt.as_ref()
        .map_or_else(|| default.to_string(), |val| val.to_string())
}
