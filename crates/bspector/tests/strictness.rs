mod common;

use bspector::Strictness;
use common::{assert_public_name_contract, assert_rejects};
use proptest::prelude::*;
use std::str::FromStr;

proptest! {
    #[test]
    fn strictness_round_trip(index in 0usize..Strictness::ALL.len()) {
        let level = Strictness::ALL[index];
        let parsed = Strictness::from_str(&level.to_string()).expect("level must parse");
        prop_assert_eq!(level, parsed);
    }
}

#[test]
fn strictness_names_cover_exact_closed_set() {
    assert_eq!(4, Strictness::ALL.len());
    assert_public_name_contract(&Strictness::ALL);
}

#[test]
fn strictness_rejects_names_outside_closed_set() {
    assert_rejects::<Strictness>(&["", "Default", "STRICT", "medium", "none"]);
}

#[test]
fn strictness_pascal_case_form_is_rejected_for_every_variant() {
    for level in Strictness::ALL {
        let pascal = format!("{level:?}");
        assert!(
            Strictness::from_str(&pascal).is_err(),
            "accepted PascalCase form {pascal:?}"
        );
    }
}

#[test]
fn strictness_variants_are_ordered_by_enforcement_level() {
    assert!(Strictness::Lax < Strictness::Default);
    assert!(Strictness::Default < Strictness::Strict);
    assert!(Strictness::Strict < Strictness::Sealed);
}

#[test]
fn strictness_stable_names_are_kebab_case() {
    for level in Strictness::ALL {
        let name = level.stable_name();
        assert!(!name.is_empty());
        assert!(!name.contains('_'));
        assert!(!name.contains(' '));
        assert_eq!(name, name.to_lowercase());
    }
}
