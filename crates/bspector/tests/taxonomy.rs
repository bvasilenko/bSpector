mod common;

use bspector::VulnerabilityPattern;
use common::{assert_public_name_contract, assert_rejects};
use proptest::prelude::*;
use std::str::FromStr;

proptest! {
    #[test]
    fn pattern_round_trip(index in 0usize..VulnerabilityPattern::ALL.len()) {
        let pattern = VulnerabilityPattern::ALL[index];
        let parsed = VulnerabilityPattern::from_str(&pattern.to_string())
            .expect("pattern must parse");
        prop_assert_eq!(pattern, parsed);
    }
}

#[test]
fn pattern_names_cover_exact_closed_set() {
    assert_eq!(12, VulnerabilityPattern::ALL.len());
    assert_public_name_contract(&VulnerabilityPattern::ALL);
}

#[test]
fn pattern_rejects_names_outside_closed_set() {
    assert_rejects::<VulnerabilityPattern>(&[
        "",
        "unknown",
        "prompt_injection",
        "prompt injection",
        "supply_chain",
        "rogue_agent",
    ]);
}

#[test]
fn pattern_pascal_case_form_is_rejected_for_every_variant() {
    for pattern in VulnerabilityPattern::ALL {
        let pascal = format!("{pattern:?}");
        assert!(
            VulnerabilityPattern::from_str(&pascal).is_err(),
            "accepted PascalCase form {pascal:?}"
        );
    }
}

#[test]
fn pattern_stable_names_are_kebab_case() {
    for pattern in VulnerabilityPattern::ALL {
        let name = pattern.stable_name();
        assert!(!name.is_empty());
        assert!(!name.contains('_'));
        assert!(!name.contains(' '));
        assert_eq!(name, name.to_lowercase());
    }
}
