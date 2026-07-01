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

#![cfg(feature = "errors")]

use bel7_cli::{
    ExitCode, ExitCodeExt, ExitCodeProvider, ExitOutcome, Outcome, PARTIAL_SUCCESS_I32,
    PARTIAL_SUCCESS_U8, codes,
};
use thiserror::Error;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
#[cfg(unix)]
use std::process::ExitStatus;

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

// Shared by the partial-success collision test and the sysexits
// round-trip test.
const ALL_EXIT_CODES: &[ExitCode] = &[
    ExitCode::Ok,
    ExitCode::Usage,
    ExitCode::DataErr,
    ExitCode::NoInput,
    ExitCode::NoUser,
    ExitCode::NoHost,
    ExitCode::Unavailable,
    ExitCode::Software,
    ExitCode::OsErr,
    ExitCode::OsFile,
    ExitCode::CantCreat,
    ExitCode::IoErr,
    ExitCode::TempFail,
    ExitCode::Protocol,
    ExitCode::NoPerm,
    ExitCode::Config,
];

#[test]
fn exit_code_provider_maps_each_variant() {
    assert_eq!(TestError::NotFound.exit_code(), ExitCode::NoInput);
    assert_eq!(TestError::Denied.exit_code(), ExitCode::NoPerm);
}

#[test]
fn partial_success_constant_is_three() {
    assert_eq!(PARTIAL_SUCCESS_I32, 3);
    assert_eq!(codes::PARTIAL_SUCCESS, 3);
    assert_eq!(codes::PARTIAL_SUCCESS, PARTIAL_SUCCESS_I32);
}

#[test]
fn partial_success_u8_mirrors_i32_constant() {
    assert_eq!(PARTIAL_SUCCESS_U8, 3);
    assert_eq!(codes::PARTIAL_SUCCESS_U8, PARTIAL_SUCCESS_U8);
    assert_eq!(i32::from(PARTIAL_SUCCESS_U8), PARTIAL_SUCCESS_I32);
}

#[test]
fn partial_success_does_not_collide_with_sysexits_constants() {
    for ec in ALL_EXIT_CODES {
        assert_ne!(
            ec.to_i32(),
            PARTIAL_SUCCESS_I32,
            "partial-success collides with sysexits variant {ec:?}"
        );
    }
}

#[test]
fn outcome_success_exit_code_is_zero() {
    let o: Outcome<TestError> = Outcome::Success;
    assert_eq!(o.exit_code_i32(), 0);
}

#[test]
fn outcome_partial_success_exit_code_is_three() {
    let o: Outcome<TestError> = Outcome::PartialSuccess;
    assert_eq!(o.exit_code_i32(), PARTIAL_SUCCESS_I32);
}

#[test]
fn outcome_failure_exit_code_delegates_to_exit_code_provider() {
    let o: Outcome<TestError> = Outcome::Failure(TestError::NotFound);
    assert_eq!(o.exit_code_i32(), ExitCode::NoInput.to_i32());
    let o: Outcome<TestError> = Outcome::Failure(TestError::Denied);
    assert_eq!(o.exit_code_i32(), ExitCode::NoPerm.to_i32());
}

#[test]
fn outcome_into_result_u8_collapses_success_and_partial_to_ok() {
    let success: Outcome<TestError> = Outcome::Success;
    assert_eq!(success.into_result_u8().unwrap(), 0);

    let partial: Outcome<TestError> = Outcome::PartialSuccess;
    assert_eq!(partial.into_result_u8().unwrap(), 3);
}

#[test]
fn outcome_into_result_u8_propagates_failure_unchanged() {
    let failure: Outcome<TestError> = Outcome::Failure(TestError::Denied);
    let err = failure.into_result_u8().unwrap_err();
    assert!(matches!(err, TestError::Denied));
    assert_eq!(err.exit_code(), ExitCode::NoPerm);
}

#[test]
fn outcome_partial_if_maps_bool_to_variant() {
    let degraded: Outcome<TestError> = Outcome::partial_if(true);
    assert!(degraded.is_partial_success());

    let clean: Outcome<TestError> = Outcome::partial_if(false);
    assert!(clean.is_success());
}

#[test]
fn exit_outcome_max_selects_worst() {
    assert!(ExitOutcome::Success < ExitOutcome::PartialSuccess);
    assert!(ExitOutcome::PartialSuccess < ExitOutcome::Failure(1));

    let outcomes = [
        ExitOutcome::Success,
        ExitOutcome::PartialSuccess,
        ExitOutcome::Success,
    ];
    assert_eq!(
        outcomes.iter().max().copied(),
        Some(ExitOutcome::PartialSuccess)
    );
}

#[test]
fn outcome_exit_code_i32_does_not_consume_outcome() {
    let outcome: Outcome<TestError> = Outcome::PartialSuccess;
    let first = outcome.exit_code_i32();
    let second = outcome.exit_code_i32();
    assert_eq!(first, second);
    assert_eq!(first, PARTIAL_SUCCESS_I32);
}

#[test]
fn exit_outcome_from_code_zero_is_success() {
    assert_eq!(ExitOutcome::from_code(0), ExitOutcome::Success);
}

#[test]
fn exit_outcome_from_code_three_is_partial_success() {
    assert_eq!(
        ExitOutcome::from_code(PARTIAL_SUCCESS_I32),
        ExitOutcome::PartialSuccess
    );
}

#[test]
fn exit_outcome_from_code_other_is_failure_with_code() {
    assert_eq!(ExitOutcome::from_code(64), ExitOutcome::Failure(64));
    assert_eq!(ExitOutcome::from_code(75), ExitOutcome::Failure(75));
    assert_eq!(ExitOutcome::from_code(-1), ExitOutcome::Failure(-1));
    assert_eq!(ExitOutcome::from_code(1), ExitOutcome::Failure(1));
}

#[test]
fn exit_outcome_is_success_shaped_truth_table() {
    assert!(ExitOutcome::Success.is_success_shaped());
    assert!(ExitOutcome::PartialSuccess.is_success_shaped());
    assert!(!ExitOutcome::Failure(1).is_success_shaped());
    assert!(!ExitOutcome::Failure(64).is_success_shaped());
    assert!(!ExitOutcome::Failure(-1).is_success_shaped());
}

#[test]
fn exit_outcome_code_round_trips_with_from_code() {
    for n in [0i32, PARTIAL_SUCCESS_I32, 1, 64, 75, -1, 255] {
        assert_eq!(ExitOutcome::from_code(n).code(), n);
    }
}

#[test]
fn exit_outcome_failure_carries_original_sysexits_code() {
    for ec in ALL_EXIT_CODES {
        let code = ec.to_i32();
        if code == 0 || code == PARTIAL_SUCCESS_I32 {
            continue;
        }
        assert_eq!(ExitOutcome::from_code(code), ExitOutcome::Failure(code));
    }
}

// On Unix `ExitStatus::code() == None` means the child was killed by a
// signal. `from_status` encodes those as `Failure(128 + signum)` (the
// shell convention) so `SIGTERM` stays distinguishable from `SIGINT` and
// from a literal `exit(143)`. Windows lacks the equivalent primitive;
// downstream integration tests cover real-process behaviour there.
#[cfg(unix)]
#[test]
fn exit_outcome_from_status_encodes_sigkill_as_one_thirty_seven() {
    let killed_by_sigkill = ExitStatus::from_raw(9);
    assert!(killed_by_sigkill.code().is_none());
    let outcome = ExitOutcome::from_status(&killed_by_sigkill);
    assert_eq!(outcome, ExitOutcome::Failure(137));
    assert!(!outcome.is_success_shaped());
}

#[cfg(unix)]
#[test]
fn exit_outcome_from_status_distinguishes_signal_kills() {
    let by_sigterm = ExitStatus::from_raw(15);
    let by_sigint = ExitStatus::from_raw(2);
    let by_sigkill = ExitStatus::from_raw(9);
    assert_eq!(
        ExitOutcome::from_status(&by_sigterm),
        ExitOutcome::Failure(143)
    );
    assert_eq!(
        ExitOutcome::from_status(&by_sigint),
        ExitOutcome::Failure(130)
    );
    assert_eq!(
        ExitOutcome::from_status(&by_sigkill),
        ExitOutcome::Failure(137)
    );
}

#[cfg(unix)]
#[test]
fn exit_outcome_from_status_clean_exits_route_through_from_code() {
    let raw_zero = ExitStatus::from_raw(0);
    assert_eq!(ExitOutcome::from_status(&raw_zero), ExitOutcome::Success);

    // `ExitStatusExt::from_raw` takes a wait-status word: the exit code lives in the high byte.
    let raw_partial = ExitStatus::from_raw(PARTIAL_SUCCESS_I32 << 8);
    assert_eq!(
        ExitOutcome::from_status(&raw_partial),
        ExitOutcome::PartialSuccess
    );

    let raw_usage = ExitStatus::from_raw(64 << 8);
    assert_eq!(
        ExitOutcome::from_status(&raw_usage),
        ExitOutcome::Failure(64)
    );
}

#[test]
fn outcome_is_success_true_only_on_success_arm() {
    assert!(Outcome::<TestError>::Success.is_success());
    assert!(!Outcome::<TestError>::PartialSuccess.is_success());
    assert!(!Outcome::Failure(TestError::Denied).is_success());
}

#[test]
fn outcome_is_partial_success_true_only_on_partial_success_arm() {
    assert!(!Outcome::<TestError>::Success.is_partial_success());
    assert!(Outcome::<TestError>::PartialSuccess.is_partial_success());
    assert!(!Outcome::Failure(TestError::Denied).is_partial_success());
}

#[test]
fn outcome_is_failure_true_only_on_failure_arm() {
    assert!(!Outcome::<TestError>::Success.is_failure());
    assert!(!Outcome::<TestError>::PartialSuccess.is_failure());
    assert!(Outcome::Failure(TestError::Denied).is_failure());
}

#[test]
fn outcome_display_success_format_is_stable() {
    let o: Outcome<TestError> = Outcome::Success;
    assert_eq!(o.to_string(), "success (exit 0)");
}

#[test]
fn outcome_display_partial_success_format_is_stable() {
    let o: Outcome<TestError> = Outcome::PartialSuccess;
    assert_eq!(o.to_string(), "partial success (exit 3)");
}

#[test]
fn outcome_display_failure_includes_exit_code_and_error_message() {
    let o: Outcome<TestError> = Outcome::Failure(TestError::NotFound);
    let expected = format!("failure (exit {}): not found", ExitCode::NoInput.to_i32());
    assert_eq!(o.to_string(), expected);

    let o: Outcome<TestError> = Outcome::Failure(TestError::Denied);
    let expected = format!("failure (exit {}): denied", ExitCode::NoPerm.to_i32());
    assert_eq!(o.to_string(), expected);
}

#[test]
fn exit_outcome_is_success_true_only_on_success_arm() {
    assert!(ExitOutcome::Success.is_success());
    assert!(!ExitOutcome::PartialSuccess.is_success());
    assert!(!ExitOutcome::Failure(64).is_success());
}

#[test]
fn exit_outcome_is_partial_success_true_only_on_partial_success_arm() {
    assert!(!ExitOutcome::Success.is_partial_success());
    assert!(ExitOutcome::PartialSuccess.is_partial_success());
    assert!(!ExitOutcome::Failure(64).is_partial_success());
}

#[test]
fn exit_outcome_is_failure_true_only_on_failure_arm() {
    assert!(!ExitOutcome::Success.is_failure());
    assert!(!ExitOutcome::PartialSuccess.is_failure());
    assert!(ExitOutcome::Failure(64).is_failure());
}

#[test]
fn exit_outcome_display_success_format_is_stable() {
    assert_eq!(ExitOutcome::Success.to_string(), "success (exit 0)");
}

#[test]
fn exit_outcome_display_partial_success_format_is_stable() {
    assert_eq!(
        ExitOutcome::PartialSuccess.to_string(),
        "partial success (exit 3)"
    );
}

#[test]
fn exit_outcome_display_failure_includes_exit_code() {
    assert_eq!(ExitOutcome::Failure(64).to_string(), "failure (exit 64)");
    assert_eq!(ExitOutcome::Failure(-1).to_string(), "failure (exit -1)");
    assert_eq!(ExitOutcome::Failure(137).to_string(), "failure (exit 137)");
}

#[test]
fn exit_outcome_display_renders_non_canonical_failure_verbatim() {
    assert_eq!(ExitOutcome::Failure(0).to_string(), "failure (exit 0)");
    assert_eq!(
        ExitOutcome::Failure(PARTIAL_SUCCESS_I32).to_string(),
        "failure (exit 3)"
    );
}

#[test]
fn exit_outcome_canonicalize_collapses_failure_zero_to_success() {
    assert_eq!(ExitOutcome::Failure(0).canonicalize(), ExitOutcome::Success);
}

#[test]
fn exit_outcome_canonicalize_collapses_failure_partial_to_partial_success() {
    assert_eq!(
        ExitOutcome::Failure(PARTIAL_SUCCESS_I32).canonicalize(),
        ExitOutcome::PartialSuccess
    );
}

#[test]
fn exit_outcome_canonicalize_leaves_canonical_arms_unchanged() {
    assert_eq!(ExitOutcome::Success.canonicalize(), ExitOutcome::Success);
    assert_eq!(
        ExitOutcome::PartialSuccess.canonicalize(),
        ExitOutcome::PartialSuccess
    );
    assert_eq!(
        ExitOutcome::Failure(64).canonicalize(),
        ExitOutcome::Failure(64)
    );
    assert_eq!(
        ExitOutcome::Failure(-1).canonicalize(),
        ExitOutcome::Failure(-1)
    );
}

mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_exit_outcome_code_round_trips_with_from_code(n in any::<i32>()) {
            prop_assert_eq!(ExitOutcome::from_code(n).code(), n);
        }

        #[test]
        fn prop_exit_outcome_success_shaped_iff_zero_or_partial(n in any::<i32>()) {
            let shaped = ExitOutcome::from_code(n).is_success_shaped();
            let expected = n == 0 || n == PARTIAL_SUCCESS_I32;
            prop_assert_eq!(shaped, expected);
        }

        #[test]
        fn prop_exit_outcome_from_code_arm_matches_input(n in any::<i32>()) {
            match ExitOutcome::from_code(n) {
                ExitOutcome::Success => prop_assert_eq!(n, 0),
                ExitOutcome::PartialSuccess => prop_assert_eq!(n, PARTIAL_SUCCESS_I32),
                ExitOutcome::Failure(c) => {
                    prop_assert_eq!(c, n);
                    prop_assert!(n != 0 && n != PARTIAL_SUCCESS_I32);
                }
            }
        }

        #[test]
        fn prop_canonicalize_failure_equals_from_code_of_inner(n in any::<i32>()) {
            prop_assert_eq!(
                ExitOutcome::Failure(n).canonicalize(),
                ExitOutcome::from_code(n)
            );
        }

        #[test]
        fn prop_canonicalize_is_idempotent(n in any::<i32>()) {
            let once = ExitOutcome::Failure(n).canonicalize();
            prop_assert_eq!(once.canonicalize(), once);
        }
    }
}
