mod common;

use bspector::RiskScoreBand;
use common::{assert_public_name_contract, assert_rejects};
use proptest::prelude::*;
use std::str::FromStr;

proptest! {
    #[test]
    fn risk_band_round_trip(index in 0usize..RiskScoreBand::ALL.len()) {
        let band = RiskScoreBand::ALL[index];
        let parsed = RiskScoreBand::from_str(&band.to_string()).expect("band must parse");
        prop_assert_eq!(band, parsed);
    }
}

#[test]
fn risk_band_names_cover_exact_closed_set() {
    assert_eq!(4, RiskScoreBand::ALL.len());
    assert_public_name_contract(&RiskScoreBand::ALL);
}

#[test]
fn risk_band_default_threshold_is_high() {
    assert_eq!(RiskScoreBand::High, RiskScoreBand::DEFAULT_THRESHOLD);
}

#[test]
fn risk_band_rejects_names_outside_closed_set() {
    assert_rejects::<RiskScoreBand>(&["", "High", "CRITICAL", "info", "none"]);
}

#[test]
fn risk_band_pascal_case_form_is_rejected_for_every_variant() {
    for band in RiskScoreBand::ALL {
        let pascal = format!("{band:?}");
        assert!(
            RiskScoreBand::from_str(&pascal).is_err(),
            "accepted PascalCase form {pascal:?}"
        );
    }
}

#[test]
fn risk_band_variants_are_ordered_by_severity() {
    assert!(RiskScoreBand::Low < RiskScoreBand::Medium);
    assert!(RiskScoreBand::Medium < RiskScoreBand::High);
    assert!(RiskScoreBand::High < RiskScoreBand::Critical);
}

#[test]
fn risk_band_stable_names_are_kebab_case() {
    for band in RiskScoreBand::ALL {
        let name = band.stable_name();
        assert!(!name.is_empty());
        assert!(!name.contains('_'));
        assert!(!name.contains(' '));
        assert_eq!(name, name.to_lowercase());
    }
}
