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

//! Test helper utilities.

use std::process::Command;

/// Environment variables used for shell detection.
pub const SHELL_DETECTION_ENV_VARS: &[&str] = &[
    "SHELL",
    "NU_VERSION",
    "FISH_VERSION",
    "ZSH_VERSION",
    "BASH_VERSION",
    "PSModulePath",
];

/// Extension trait for `std::process::Command` to help with shell detection tests.
pub trait CommandShellExt {
    /// Clears environment variables used for shell detection.
    ///
    /// This removes `SHELL`, `NU_VERSION`, `FISH_VERSION`, `ZSH_VERSION`,
    /// `BASH_VERSION`, and `PSModulePath` to ensure consistent shell detection
    /// behavior in tests.
    fn clear_shell_detection_env(&mut self) -> &mut Self;
}

impl CommandShellExt for Command {
    fn clear_shell_detection_env(&mut self) -> &mut Self {
        for var in SHELL_DETECTION_ENV_VARS {
            self.env_remove(var);
        }
        self
    }
}
