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

use bel7_cli::ArgMatchesExt;
use clap::{Arg, Command};

fn test_command() -> Command {
    Command::new("test")
        .arg(Arg::new("name").long("name").required(true))
        .arg(Arg::new("count").long("count"))
}

#[test]
fn test_required_str() {
    let matches = test_command().get_matches_from(["test", "--name", "foo"]);
    assert_eq!(matches.required_str("name"), "foo");
}

#[test]
fn test_optional_str() {
    let matches = test_command().get_matches_from(["test", "--name", "foo"]);
    assert!(matches.optional_str("count").is_none());
}

#[test]
fn test_parse_optional() {
    let cmd = Command::new("test")
        .arg(Arg::new("name").long("name").required(true))
        .arg(Arg::new("count").long("count"));

    let matches = cmd.get_matches_from(["test", "--name", "foo", "--count", "42"]);
    let count: Option<i32> = matches.parse_optional("count").unwrap();
    assert_eq!(count, Some(42));
}

#[test]
fn test_required_string() {
    let matches = test_command().get_matches_from(["test", "--name", "foo"]);
    assert_eq!(matches.required_string("name"), "foo".to_string());
}

#[test]
fn test_optional_string_present() {
    let matches = test_command().get_matches_from(["test", "--name", "foo", "--count", "bar"]);
    assert_eq!(matches.optional_string("count"), Some("bar".to_string()));
}

#[test]
fn test_optional_string_missing() {
    let matches = test_command().get_matches_from(["test", "--name", "foo"]);
    assert_eq!(matches.optional_string("count"), None);
}

#[test]
fn test_optional_str_present() {
    let matches = test_command().get_matches_from(["test", "--name", "foo", "--count", "bar"]);
    assert_eq!(matches.optional_str("count"), Some("bar"));
}

#[test]
fn test_parse_required() {
    let cmd = Command::new("test").arg(Arg::new("count").long("count").required(true));

    let matches = cmd.get_matches_from(["test", "--count", "42"]);
    let count: i32 = matches.parse_required("count").unwrap();
    assert_eq!(count, 42);
}

#[test]
fn test_parse_required_invalid() {
    let cmd = Command::new("test").arg(Arg::new("count").long("count").required(true));

    let matches = cmd.get_matches_from(["test", "--count", "not_a_number"]);
    let result: Result<i32, _> = matches.parse_required("count");
    assert!(result.is_err());
}

#[test]
fn test_parse_optional_missing() {
    let matches = test_command().get_matches_from(["test", "--name", "foo"]);
    let count: Option<i32> = matches.parse_optional("count").unwrap();
    assert_eq!(count, None);
}

#[test]
fn test_parse_optional_invalid() {
    let matches = test_command().get_matches_from(["test", "--name", "foo", "--count", "bad"]);
    let result: Result<Option<i32>, _> = matches.parse_optional("count");
    assert!(result.is_err());
}

#[test]
fn test_get_typed() {
    let cmd = Command::new("test").arg(
        Arg::new("count")
            .long("count")
            .value_parser(clap::value_parser!(i32)),
    );

    let matches = cmd.get_matches_from(["test", "--count", "42"]);
    assert_eq!(matches.get_typed::<i32>("count"), Some(42));
}

#[test]
fn test_get_typed_missing() {
    let cmd = Command::new("test").arg(
        Arg::new("count")
            .long("count")
            .value_parser(clap::value_parser!(i32)),
    );

    let matches = cmd.get_matches_from(["test"]);
    assert_eq!(matches.get_typed::<i32>("count"), None);
}

#[test]
fn test_get_typed_or() {
    let cmd = Command::new("test").arg(
        Arg::new("count")
            .long("count")
            .value_parser(clap::value_parser!(i32)),
    );

    let matches = cmd.get_matches_from(["test"]);
    assert_eq!(matches.get_typed_or::<i32>("count", 99), 99);
}

#[test]
fn test_get_typed_or_present() {
    let cmd = Command::new("test").arg(
        Arg::new("count")
            .long("count")
            .value_parser(clap::value_parser!(i32)),
    );

    let matches = cmd.get_matches_from(["test", "--count", "42"]);
    assert_eq!(matches.get_typed_or::<i32>("count", 99), 42);
}
