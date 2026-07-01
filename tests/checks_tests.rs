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

use bel7_cli::{Severity, Tally};

#[test]
fn severity_orders_info_below_warning_below_error() {
    assert!(Severity::Info < Severity::Warning);
    assert!(Severity::Warning < Severity::Error);
    assert!(Severity::Info < Severity::Error);
}

#[test]
fn severity_max_picks_error_when_present() {
    let severities = [Severity::Info, Severity::Error, Severity::Warning];
    assert_eq!(severities.iter().max().copied(), Some(Severity::Error));
}

#[test]
fn severity_display_is_lowercase() {
    assert_eq!(Severity::Info.to_string(), "info");
    assert_eq!(Severity::Warning.to_string(), "warning");
    assert_eq!(Severity::Error.to_string(), "error");
}

#[test]
fn tally_new_starts_clean() {
    let tally = Tally::new();
    assert_eq!(tally.total(), 0);
    assert!(tally.is_clean());
    assert_eq!(tally.worst_severity(), None);
}

#[test]
fn tally_record_increments_only_the_matching_counter() {
    let mut tally = Tally::new();
    tally.record(Severity::Warning);
    assert_eq!(tally.info(), 0);
    assert_eq!(tally.warning(), 1);
    assert_eq!(tally.error(), 0);
    assert_eq!(tally.total(), 1);
}

#[test]
fn tally_record_accumulates_across_calls() {
    let mut tally = Tally::new();
    tally.record(Severity::Info);
    tally.record(Severity::Info);
    tally.record(Severity::Error);
    assert_eq!(tally.info(), 2);
    assert_eq!(tally.error(), 1);
    assert_eq!(tally.total(), 3);
}

#[test]
fn tally_is_clean_true_with_only_info() {
    let mut tally = Tally::new();
    tally.record(Severity::Info);
    tally.record(Severity::Info);
    assert!(tally.is_clean());
}

#[test]
fn tally_is_clean_false_with_any_warning() {
    let mut tally = Tally::new();
    tally.record(Severity::Warning);
    assert!(!tally.is_clean());
}

#[test]
fn tally_is_clean_false_with_any_error() {
    let mut tally = Tally::new();
    tally.record(Severity::Error);
    assert!(!tally.is_clean());
}

#[test]
fn tally_worst_severity_ignores_recording_order() {
    let mut tally = Tally::new();
    tally.record(Severity::Info);
    tally.record(Severity::Error);
    tally.record(Severity::Warning);
    assert_eq!(tally.worst_severity(), Some(Severity::Error));
}

#[test]
fn tally_worst_severity_reports_warning_when_no_error_present() {
    let mut tally = Tally::new();
    tally.record(Severity::Info);
    tally.record(Severity::Warning);
    assert_eq!(tally.worst_severity(), Some(Severity::Warning));
}

#[test]
fn tally_is_partial_success_worthy_truth_table() {
    let clean = Tally::new();
    assert!(!clean.is_partial_success_worthy(false));
    assert!(!clean.is_partial_success_worthy(true));

    let mut warnings_only = Tally::new();
    warnings_only.record(Severity::Warning);
    assert!(!warnings_only.is_partial_success_worthy(false));
    assert!(warnings_only.is_partial_success_worthy(true));

    let mut with_error = Tally::new();
    with_error.record(Severity::Error);
    assert!(with_error.is_partial_success_worthy(false));
    assert!(with_error.is_partial_success_worthy(true));
}

#[test]
fn tally_display_reports_all_three_counts() {
    let mut tally = Tally::new();
    tally.record(Severity::Info);
    tally.record(Severity::Warning);
    tally.record(Severity::Warning);
    assert_eq!(tally.to_string(), "1 info, 2 warning, 0 error");
}

#[cfg(feature = "serde")]
#[test]
fn severity_serde_round_trips_through_json() {
    for severity in [Severity::Info, Severity::Warning, Severity::Error] {
        let json = serde_json::to_string(&severity).unwrap();
        let round_tripped: Severity = serde_json::from_str(&json).unwrap();
        assert_eq!(round_tripped, severity);
    }
}

#[cfg(feature = "serde")]
#[test]
fn severity_serde_representation_matches_display() {
    for severity in [Severity::Info, Severity::Warning, Severity::Error] {
        let json = serde_json::to_string(&severity).unwrap();
        assert_eq!(json, format!("\"{severity}\""));
    }
}

#[cfg(feature = "serde")]
#[test]
fn tally_deserializes_missing_fields_as_zero() {
    let tally: Tally = serde_json::from_str("{}").unwrap();
    assert_eq!(tally, Tally::new());

    let tally: Tally = serde_json::from_str(r#"{"error": 2}"#).unwrap();
    assert_eq!(tally.error(), 2);
    assert_eq!(tally.info(), 0);
    assert_eq!(tally.warning(), 0);
}

#[cfg(feature = "serde")]
#[test]
fn tally_serde_round_trips_through_json() {
    let mut tally = Tally::new();
    tally.record(Severity::Info);
    tally.record(Severity::Error);

    let json = serde_json::to_string(&tally).unwrap();
    let round_tripped: Tally = serde_json::from_str(&json).unwrap();
    assert_eq!(round_tripped, tally);
}

mod proptests {
    use super::*;
    use proptest::prelude::*;

    fn tally_with_counts(info: u32, warning: u32, error: u32) -> Tally {
        let mut tally = Tally::new();
        for _ in 0..info {
            tally.record(Severity::Info);
        }
        for _ in 0..warning {
            tally.record(Severity::Warning);
        }
        for _ in 0..error {
            tally.record(Severity::Error);
        }
        tally
    }

    proptest! {
        #[test]
        fn prop_is_partial_success_worthy_matches_formula(
            info in 0u32..20,
            warning in 0u32..20,
            error in 0u32..20,
            strict in any::<bool>(),
        ) {
            let tally = tally_with_counts(info, warning, error);
            let expected = error > 0 || (strict && warning > 0);
            prop_assert_eq!(tally.is_partial_success_worthy(strict), expected);
        }

        #[test]
        fn prop_total_sums_recorded_counts(
            info in 0u32..20,
            warning in 0u32..20,
            error in 0u32..20,
        ) {
            let tally = tally_with_counts(info, warning, error);
            prop_assert_eq!(tally.total(), info + warning + error);
        }

        #[test]
        fn prop_is_clean_iff_no_warning_or_error(
            info in 0u32..20,
            warning in 0u32..20,
            error in 0u32..20,
        ) {
            let tally = tally_with_counts(info, warning, error);
            prop_assert_eq!(tally.is_clean(), warning == 0 && error == 0);
        }

        #[test]
        fn prop_worst_severity_agrees_with_naive_max(
            info in 0u32..5,
            warning in 0u32..5,
            error in 0u32..5,
        ) {
            let tally = tally_with_counts(info, warning, error);

            let mut recorded = Vec::new();
            recorded.extend(std::iter::repeat_n(Severity::Info, info as usize));
            recorded.extend(std::iter::repeat_n(Severity::Warning, warning as usize));
            recorded.extend(std::iter::repeat_n(Severity::Error, error as usize));
            let expected = recorded.into_iter().max();

            prop_assert_eq!(tally.worst_severity(), expected);
        }
    }
}
