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

//! Common CLI utilities for Rust command-line applications.
//!
//! This crate provides:
//!
//! - Colored console output helpers (success, error, warning, info)
//! - String truncation for display
//! - Table styling utilities (requires `tables` feature)
//! - Clap argument helpers (requires `clap` feature)
//!
//! # Features
//!
//! - `tables` - Enables table styling with `tabled`
//! - `clap` - Enables clap argument helper extensions
//! - `errors` - Enables exit code mapping with `sysexits`
//! - `full` - Enables all features

#[cfg(feature = "errors")]
mod errors;

mod output;
mod truncate;

#[cfg(feature = "tables")]
mod tables;

#[cfg(feature = "clap")]
mod clap_ext;

pub use output::*;
pub use truncate::*;

#[cfg(feature = "tables")]
pub use tables::*;

#[cfg(feature = "clap")]
pub use clap_ext::*;

#[cfg(feature = "errors")]
pub use errors::*;
