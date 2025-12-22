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

//! Clap argument parsing extensions.

use clap::ArgMatches;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Extension trait for `clap::ArgMatches` with convenient accessor methods.
pub trait ArgMatchesExt {
    /// Gets a required string argument, panics if missing.
    fn required_str(&self, name: &str) -> &str;

    /// Gets an optional string argument.
    fn optional_str(&self, name: &str) -> Option<&str>;

    /// Gets a required string argument as an owned String.
    fn required_string(&self, name: &str) -> String;

    /// Gets an optional string argument as an owned String.
    fn optional_string(&self, name: &str) -> Option<String>;

    /// Parses a required argument into the target type.
    fn parse_required<T>(&self, name: &str) -> Result<T, ArgParseError>
    where
        T: FromStr,
        T::Err: Display;

    /// Parses an optional argument into the target type.
    fn parse_optional<T>(&self, name: &str) -> Result<Option<T>, ArgParseError>
    where
        T: FromStr,
        T::Err: Display;

    /// Gets a typed argument that was already parsed by clap.
    fn get_typed<T: Clone + Send + Sync + 'static>(&self, name: &str) -> Option<T>;

    /// Gets a typed argument with a default value.
    fn get_typed_or<T: Clone + Send + Sync + 'static>(&self, name: &str, default: T) -> T;
}

/// Error type for argument parsing failures.
#[derive(Debug)]
pub struct ArgParseError {
    /// The argument name that failed to parse.
    pub name: String,
    /// The error message.
    pub message: String,
}

impl Display for ArgParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid value for '{}': {}", self.name, self.message)
    }
}

impl Error for ArgParseError {}

impl ArgMatchesExt for ArgMatches {
    fn required_str(&self, name: &str) -> &str {
        self.get_one::<String>(name)
            .map(|s| s.as_str())
            .unwrap_or_else(|| panic!("Required argument '{}' not provided", name))
    }

    fn optional_str(&self, name: &str) -> Option<&str> {
        self.get_one::<String>(name).map(|s| s.as_str())
    }

    fn required_string(&self, name: &str) -> String {
        self.required_str(name).to_string()
    }

    fn optional_string(&self, name: &str) -> Option<String> {
        self.optional_str(name).map(|s| s.to_string())
    }

    fn parse_required<T>(&self, name: &str) -> Result<T, ArgParseError>
    where
        T: FromStr,
        T::Err: Display,
    {
        let value = self.required_str(name);
        value.parse::<T>().map_err(|e| ArgParseError {
            name: name.to_string(),
            message: e.to_string(),
        })
    }

    fn parse_optional<T>(&self, name: &str) -> Result<Option<T>, ArgParseError>
    where
        T: FromStr,
        T::Err: Display,
    {
        match self.optional_str(name) {
            Some(value) => value.parse::<T>().map(Some).map_err(|e| ArgParseError {
                name: name.to_string(),
                message: e.to_string(),
            }),
            None => Ok(None),
        }
    }

    fn get_typed<T: Clone + Send + Sync + 'static>(&self, name: &str) -> Option<T> {
        self.get_one::<T>(name).cloned()
    }

    fn get_typed_or<T: Clone + Send + Sync + 'static>(&self, name: &str, default: T) -> T {
        self.get_typed(name).unwrap_or(default)
    }
}
