use bspector::{BspectorError, substrate_input::SubstrateInput};
use proptest::prelude::*;

fn make_input(artefact: &str) -> Result<SubstrateInput, BspectorError> {
    SubstrateInput::new(artefact.to_owned(), None, None, false, false, None, None)
}

proptest! {
    #[test]
    fn artefact_acceptance_mirrors_non_blank_check(artefact in ".*") {
        let is_blank = artefact.trim().is_empty();
        let result = make_input(&artefact);
        if is_blank {
            prop_assert!(result.is_err(), "blank artefact should be rejected: {artefact:?}");
        } else {
            prop_assert!(result.is_ok(), "non-blank artefact should be accepted: {artefact:?}");
        }
    }
}

#[test]
fn substrate_input_accepts_valid_artefact_paths() {
    for artefact in [
        "./README.md",
        "https://example.com/skill.md",
        "skill.md",
        "3",
    ] {
        let input = make_input(artefact).expect("valid artefact accepted");
        assert_eq!(artefact, input.artefact);
    }
}

#[test]
fn substrate_input_rejects_empty_artefact() {
    let error = make_input("").expect_err("empty artefact rejected");
    assert!(matches!(error, BspectorError::ArtefactInputMalformed(_)));
}

#[test]
fn substrate_input_rejects_whitespace_only_artefact() {
    for blank in [" ", "   ", "\t", "\n"] {
        let error = make_input(blank).expect_err("blank artefact rejected");
        assert!(matches!(error, BspectorError::ArtefactInputMalformed(_)));
    }
}

#[test]
fn substrate_input_accepts_artefact_with_surrounding_whitespace() {
    for artefact in [" skill.md", "skill.md ", " ./path/to/skill.md "] {
        let input = make_input(artefact).expect("artefact with surrounding whitespace accepted");
        assert_eq!(artefact, input.artefact);
    }
}

#[test]
fn substrate_input_optional_fields_default_to_none_when_absent() {
    let input = make_input("./skill.md").expect("valid artefact");
    assert!(input.artefact_shape.is_none());
    assert!(input.strictness.is_none());
    assert!(!input.llm_stage);
    assert!(!input.osv_online);
    assert!(input.manifest.is_none());
    assert!(input.reason.is_none());
}

#[test]
fn substrate_input_boolean_flags_are_stored_independently() {
    let llm_only = SubstrateInput::new("artefact".to_owned(), None, None, true, false, None, None)
        .expect("valid");
    assert!(llm_only.llm_stage);
    assert!(!llm_only.osv_online);

    let osv_only = SubstrateInput::new("artefact".to_owned(), None, None, false, true, None, None)
        .expect("valid");
    assert!(!osv_only.llm_stage);
    assert!(osv_only.osv_online);
}

#[test]
fn substrate_input_preserves_all_fields() {
    use std::path::PathBuf;

    let input = SubstrateInput::new(
        "./skill.md".to_owned(),
        Some("skill-md".to_owned()),
        Some("strict".to_owned()),
        true,
        false,
        Some(PathBuf::from("manifest.toml")),
        Some("review requested".to_owned()),
    )
    .expect("valid input");

    assert_eq!("./skill.md", input.artefact);
    assert_eq!(Some("skill-md".to_owned()), input.artefact_shape);
    assert_eq!(Some("strict".to_owned()), input.strictness);
    assert!(input.llm_stage);
    assert!(!input.osv_online);
    assert_eq!(Some(PathBuf::from("manifest.toml")), input.manifest);
    assert_eq!(Some("review requested".to_owned()), input.reason);
}

#[test]
fn substrate_input_manifest_accepts_absolute_path() {
    use std::path::PathBuf;

    let path = PathBuf::from("/etc/bspector/manifest.toml");
    let input = SubstrateInput::new(
        "./skill.md".to_owned(),
        None,
        None,
        false,
        false,
        Some(path.clone()),
        None,
    )
    .expect("valid");
    assert_eq!(Some(path), input.manifest);
}
