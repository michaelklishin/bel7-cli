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

use bel7_cli::{ExitCode, ExitCodeProvider};
use thiserror::Error;

#[derive(Error, Debug)]
enum TestError {
    #[error("not found")]
    NotFound,
    #[error("denied")]
    Denied,
}

impl ExitCodeProvider for TestError {
    fn exit_code(&self) -> ExitCode {
        match self {
            TestError::NotFound => ExitCode::NoInput,
            TestError::Denied => ExitCode::NoPerm,
        }
    }
}

#[test]
fn test_exit_code_mapping() {
    assert_eq!(TestError::NotFound.exit_code(), ExitCode::NoInput);
    assert_eq!(TestError::Denied.exit_code(), ExitCode::NoPerm);
}
