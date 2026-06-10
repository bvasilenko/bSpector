mod common;

use bspector::ScanState;
use bsuite_core::ExitCode;
use common::{assert_public_name_contract, assert_rejects};
use proptest::prelude::*;
use std::str::FromStr;

proptest! {
    #[test]
    fn scan_state_round_trip(index in 0usize..ScanState::ALL.len()) {
        let state = ScanState::ALL[index];
        let parsed = ScanState::from_str(&state.to_string()).expect("state must parse");
        prop_assert_eq!(state, parsed);
    }
}

#[test]
fn scan_state_names_cover_exact_closed_set() {
    assert_eq!(4, ScanState::ALL.len());
    assert_public_name_contract(&ScanState::ALL);
}

#[test]
fn scan_state_exit_codes_match_contract() {
    let cases = [
        (ScanState::Safe, ExitCode::Success, 0),
        (ScanState::Unsafe, ExitCode::Finding, 1),
        (ScanState::Malformed, ExitCode::Usage, 64),
        (ScanState::InternalError, ExitCode::InternalError, 2),
    ];

    for (state, expected_code, expected_raw) in cases {
        let code: ExitCode = state.into();
        assert_eq!(expected_code, code, "state {state:?} exit code mismatch");
        assert_eq!(
            expected_raw,
            code.as_i32(),
            "state {state:?} raw code mismatch"
        );
    }
}

#[test]
fn scan_state_exit_code_mapping_is_injective() {
    let raw_codes: Vec<i32> = ScanState::ALL
        .iter()
        .map(|&state| {
            let code: ExitCode = state.into();
            code.as_i32()
        })
        .collect();

    let mut sorted = raw_codes.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(
        ScanState::ALL.len(),
        sorted.len(),
        "two or more scan states produce the same exit code"
    );
}

#[test]
fn scan_state_rejects_names_outside_closed_set() {
    assert_rejects::<ScanState>(&["", "clean", "finding", "Safe", "Unsafe", "internal_error"]);
}

#[test]
fn scan_state_pascal_case_form_is_rejected_for_every_variant() {
    for state in ScanState::ALL {
        let pascal = format!("{state:?}");
        assert!(
            ScanState::from_str(&pascal).is_err(),
            "accepted PascalCase form {pascal:?}"
        );
    }
}

#[test]
fn scan_state_stable_names_are_kebab_case() {
    for state in ScanState::ALL {
        let name = state.stable_name();
        assert!(!name.is_empty());
        assert!(!name.contains('_'));
        assert!(!name.contains(' '));
        assert_eq!(name, name.to_lowercase());
    }
}
