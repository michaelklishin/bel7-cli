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

#![cfg(feature = "completions")]

use std::collections::HashSet;

use bel7_cli::{CompletionShell, ParseShellError, generate_completions};
use clap::{Arg, Command, ValueEnum};

#[test]
fn test_all_shells_parse_from_str() {
    let cases = [
        ("bash", CompletionShell::Bash),
        ("zsh", CompletionShell::Zsh),
        ("fish", CompletionShell::Fish),
        ("elvish", CompletionShell::Elvish),
        ("nushell", CompletionShell::Nushell),
        ("nu", CompletionShell::Nushell),
        ("powershell", CompletionShell::PowerShell),
        ("pwsh", CompletionShell::PowerShell),
    ];

    for (input, expected) in cases {
        let parsed: CompletionShell = input.parse().unwrap();
        assert_eq!(parsed, expected);
    }
}

#[test]
fn test_parse_case_insensitive() {
    let cases = ["BASH", "Bash", "bAsH", "ZSH", "Zsh"];
    for input in cases {
        let result: Result<CompletionShell, _> = input.parse();
        assert!(result.is_ok(), "failed to parse: {}", input);
    }
}

#[test]
fn test_parse_unknown_shell_fails() {
    let result: Result<CompletionShell, ParseShellError> = "unknown".parse();
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("unknown"));
}

#[test]
fn test_display() {
    assert_eq!(CompletionShell::Bash.to_string(), "bash");
    assert_eq!(CompletionShell::Zsh.to_string(), "zsh");
    assert_eq!(CompletionShell::Fish.to_string(), "fish");
    assert_eq!(CompletionShell::Elvish.to_string(), "elvish");
    assert_eq!(CompletionShell::Nushell.to_string(), "nushell");
    assert_eq!(CompletionShell::PowerShell.to_string(), "powershell");
}

#[test]
fn test_all_returns_all_variants() {
    let all = CompletionShell::all();
    assert_eq!(all.len(), 6);
    assert!(all.contains(&CompletionShell::Bash));
    assert!(all.contains(&CompletionShell::Zsh));
    assert!(all.contains(&CompletionShell::Fish));
    assert!(all.contains(&CompletionShell::Elvish));
    assert!(all.contains(&CompletionShell::Nushell));
    assert!(all.contains(&CompletionShell::PowerShell));
}

#[test]
fn test_generate_completions_produces_output() {
    let shells = CompletionShell::all();
    for shell in shells {
        let mut cmd = Command::new("test-app").subcommand(Command::new("sub"));
        let mut output = Vec::new();
        generate_completions(*shell, &mut cmd, "test-app", &mut output);
        assert!(
            !output.is_empty(),
            "{} should produce completion output",
            shell
        );
    }
}

#[test]
fn test_bash_completions_content() {
    let mut cmd = Command::new("myapp")
        .subcommand(Command::new("list"))
        .subcommand(Command::new("show"));
    let mut output = Vec::new();
    generate_completions(CompletionShell::Bash, &mut cmd, "myapp", &mut output);
    let content = String::from_utf8(output).unwrap();
    assert!(content.contains("myapp"));
}

#[test]
fn test_zsh_completions_content() {
    let mut cmd = Command::new("myapp").subcommand(Command::new("install"));
    let mut output = Vec::new();
    generate_completions(CompletionShell::Zsh, &mut cmd, "myapp", &mut output);
    let content = String::from_utf8(output).unwrap();
    assert!(content.contains("myapp"));
}

#[test]
fn test_fish_completions_content() {
    let mut cmd = Command::new("myapp").arg(Arg::new("verbose").long("verbose"));
    let mut output = Vec::new();
    generate_completions(CompletionShell::Fish, &mut cmd, "myapp", &mut output);
    let content = String::from_utf8(output).unwrap();
    assert!(content.contains("myapp"));
}

#[test]
fn test_elvish_completions_content() {
    let mut cmd = Command::new("myapp").subcommand(Command::new("run"));
    let mut output = Vec::new();
    generate_completions(CompletionShell::Elvish, &mut cmd, "myapp", &mut output);
    let content = String::from_utf8(output).unwrap();
    assert!(content.contains("myapp"));
}

#[test]
fn test_nushell_completions_content() {
    let mut cmd = Command::new("myapp").subcommand(Command::new("build"));
    let mut output = Vec::new();
    generate_completions(CompletionShell::Nushell, &mut cmd, "myapp", &mut output);
    let content = String::from_utf8(output).unwrap();
    assert!(content.contains("myapp"));
}

#[test]
fn test_powershell_completions_content() {
    let mut cmd = Command::new("myapp").arg(Arg::new("config").short('c'));
    let mut output = Vec::new();
    generate_completions(CompletionShell::PowerShell, &mut cmd, "myapp", &mut output);
    let content = String::from_utf8(output).unwrap();
    assert!(content.contains("myapp"));
}

#[test]
fn test_value_enum_variants() {
    let variants = CompletionShell::value_variants();
    assert_eq!(variants.len(), 6);
}

#[test]
fn test_value_enum_possible_values() {
    for shell in CompletionShell::value_variants() {
        let pv = shell.to_possible_value();
        assert!(pv.is_some());
    }
}

#[test]
fn test_clone_and_copy() {
    let shell = CompletionShell::Bash;
    let cloned = shell.clone();
    let copied = shell;
    assert_eq!(shell, cloned);
    assert_eq!(shell, copied);
}

#[test]
fn test_debug() {
    let shell = CompletionShell::Fish;
    let debug = format!("{:?}", shell);
    assert!(debug.contains("Fish"));
}

#[test]
fn test_hash() {
    let mut set = HashSet::new();
    set.insert(CompletionShell::Bash);
    set.insert(CompletionShell::Zsh);
    set.insert(CompletionShell::Bash);
    assert_eq!(set.len(), 2);
}

#[test]
fn test_detect_returns_valid_shell() {
    let detected = CompletionShell::detect();
    assert!(CompletionShell::all().contains(&detected));
}

#[test]
fn test_default_is_bash() {
    assert_eq!(CompletionShell::default(), CompletionShell::Bash);
}

#[test]
fn test_parse_shell_error_clone() {
    let result: Result<CompletionShell, ParseShellError> = "invalid".parse();
    let err = result.unwrap_err();
    let cloned = err.clone();
    assert_eq!(err, cloned);
}

#[test]
fn test_parse_shell_error_debug() {
    let result: Result<CompletionShell, ParseShellError> = "unknown".parse();
    let err = result.unwrap_err();
    let debug = format!("{:?}", err);
    assert!(debug.contains("ParseShellError"));
}

mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn parse_then_display_roundtrips(shell in prop_oneof![
            Just("bash"),
            Just("zsh"),
            Just("fish"),
            Just("elvish"),
            Just("nushell"),
            Just("nu"),
            Just("powershell"),
            Just("pwsh"),
        ]) {
            let parsed: CompletionShell = shell.parse().unwrap();
            let displayed = parsed.to_string();
            let reparsed: CompletionShell = displayed.parse().unwrap();
            assert_eq!(parsed, reparsed);
        }

        #[test]
        fn unknown_shells_fail_to_parse(s in "[a-z]{1,10}") {
            let known = ["bash", "zsh", "fish", "elvish", "nushell", "nu", "powershell", "pwsh"];
            if !known.contains(&s.as_str()) {
                let result: Result<CompletionShell, _> = s.parse();
                assert!(result.is_err());
            }
        }
    }
}
