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

//! Shell completion generation utilities.

use std::env;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::path::Path;
use std::str::FromStr;

use clap::Command;
use clap::builder::PossibleValue;
use clap_complete::Shell as ClapShell;
use clap_complete::generate;
use clap_complete_nushell::Nushell;

const ALL_SHELLS: &[CompletionShell] = &[
    CompletionShell::Bash,
    CompletionShell::Zsh,
    CompletionShell::Fish,
    CompletionShell::Elvish,
    CompletionShell::Nushell,
    CompletionShell::PowerShell,
];

/// Supported shells for completion script generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CompletionShell {
    #[default]
    Bash,
    Zsh,
    Fish,
    Elvish,
    Nushell,
    PowerShell,
}

impl clap::ValueEnum for CompletionShell {
    fn value_variants<'a>() -> &'a [Self] {
        ALL_SHELLS
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Bash => PossibleValue::new("bash"),
            Self::Zsh => PossibleValue::new("zsh"),
            Self::Fish => PossibleValue::new("fish"),
            Self::Elvish => PossibleValue::new("elvish"),
            Self::Nushell => PossibleValue::new("nushell").alias("nu"),
            Self::PowerShell => PossibleValue::new("powershell").alias("pwsh"),
        })
    }
}

impl CompletionShell {
    /// Detects the current shell from environment variables.
    ///
    /// Checks in order:
    /// 1. `NU_VERSION` env var (indicates Nushell)
    /// 2. `SHELL` env var (parses the shell name from the path)
    ///
    /// Falls back to Bash if detection fails.
    #[must_use]
    pub fn detect() -> Self {
        if env::var("NU_VERSION").is_ok() {
            return Self::Nushell;
        }

        env::var("SHELL")
            .ok()
            .and_then(|s| {
                let shell_name = Path::new(&s)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&s);
                match shell_name {
                    "bash" => Some(Self::Bash),
                    "zsh" => Some(Self::Zsh),
                    "fish" => Some(Self::Fish),
                    "elvish" => Some(Self::Elvish),
                    "nu" | "nushell" => Some(Self::Nushell),
                    "pwsh" | "powershell" => Some(Self::PowerShell),
                    _ => None,
                }
            })
            .unwrap_or(Self::Bash)
    }

    /// Returns all supported shell variants.
    #[must_use]
    pub fn all() -> &'static [Self] {
        ALL_SHELLS
    }
}

impl fmt::Display for CompletionShell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Bash => "bash",
            Self::Zsh => "zsh",
            Self::Fish => "fish",
            Self::Elvish => "elvish",
            Self::Nushell => "nushell",
            Self::PowerShell => "powershell",
        };
        f.write_str(name)
    }
}

/// Error when parsing a shell name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseShellError {
    input: String,
}

impl fmt::Display for ParseShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown shell: {}", self.input)
    }
}

impl Error for ParseShellError {}

impl FromStr for CompletionShell {
    type Err = ParseShellError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bash" => Ok(Self::Bash),
            "zsh" => Ok(Self::Zsh),
            "fish" => Ok(Self::Fish),
            "elvish" => Ok(Self::Elvish),
            "nu" | "nushell" => Ok(Self::Nushell),
            "pwsh" | "powershell" => Ok(Self::PowerShell),
            _ => Err(ParseShellError { input: s.into() }),
        }
    }
}

/// Generates shell completion scripts and writes them to the given writer.
pub fn generate_completions<W: Write>(
    shell: CompletionShell,
    cmd: &mut Command,
    bin_name: &str,
    out: &mut W,
) {
    match shell {
        CompletionShell::Bash => generate(ClapShell::Bash, cmd, bin_name, out),
        CompletionShell::Zsh => generate(ClapShell::Zsh, cmd, bin_name, out),
        CompletionShell::Fish => generate(ClapShell::Fish, cmd, bin_name, out),
        CompletionShell::Elvish => generate(ClapShell::Elvish, cmd, bin_name, out),
        CompletionShell::Nushell => generate(Nushell, cmd, bin_name, out),
        CompletionShell::PowerShell => generate(ClapShell::PowerShell, cmd, bin_name, out),
    }
}

/// Generates shell completion scripts and writes them to stdout.
pub fn generate_completions_to_stdout(shell: CompletionShell, cmd: &mut Command, bin_name: &str) {
    generate_completions(shell, cmd, bin_name, &mut io::stdout());
}
