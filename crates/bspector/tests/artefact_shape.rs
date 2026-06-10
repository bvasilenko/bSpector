mod common;

use bspector::ArtefactShape;
use common::{assert_public_name_contract, assert_rejects};
use proptest::prelude::*;
use std::str::FromStr;

proptest! {
    #[test]
    fn artefact_shape_round_trip(index in 0usize..ArtefactShape::ALL.len()) {
        let shape = ArtefactShape::ALL[index];
        let parsed = ArtefactShape::from_str(&shape.to_string()).expect("shape must parse");
        prop_assert_eq!(shape, parsed);
    }
}

#[test]
fn artefact_shape_names_cover_exact_closed_set() {
    assert_eq!(7, ArtefactShape::ALL.len());
    assert_public_name_contract(&ArtefactShape::ALL);
}

#[test]
fn artefact_shape_rejects_names_outside_closed_set() {
    assert_rejects::<ArtefactShape>(&["", "skill_md", "manifest", "generic"]);
}

#[test]
fn artefact_shape_pascal_case_form_is_rejected_for_every_variant() {
    for shape in ArtefactShape::ALL {
        let pascal = format!("{shape:?}");
        assert!(
            ArtefactShape::from_str(&pascal).is_err(),
            "accepted PascalCase form {pascal:?}"
        );
    }
}

#[test]
fn artefact_shape_stable_names_are_kebab_case() {
    for shape in ArtefactShape::ALL {
        let name = shape.stable_name();
        assert!(!name.is_empty());
        assert!(!name.contains('_'));
        assert!(!name.contains(' '));
        assert_eq!(name, name.to_lowercase());
    }
}
