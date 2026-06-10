use bspector::BspectorError;
use bspector::error::process_exit_code;
use bsuite_core::{BsuiteCoreError, ExitCode};

fn assert_error_exit_code(error: BspectorError, expected: ExitCode) {
    assert_eq!(expected, error.exit_code());
    assert_eq!(
        std::process::ExitCode::from(expected.as_i32() as u8),
        error.process_exit_code(),
    );
}

#[test]
fn usage_errors_map_to_usage_exit_code() {
    assert_error_exit_code(
        BspectorError::Usage("bad arguments".to_owned()),
        ExitCode::Usage,
    );
}

#[test]
fn domain_errors_map_to_internal_error_exit_code() {
    for error in [
        BspectorError::ArtefactInputMalformed("bad artefact".to_owned()),
        BspectorError::TaxonomyUnknown("bad pattern".to_owned()),
        BspectorError::ScanStateUnknown("bad state".to_owned()),
        BspectorError::ArtefactShapeUnknown("bad shape".to_owned()),
        BspectorError::StrictnessUnknown("bad strictness".to_owned()),
        BspectorError::RiskScoreBandUnknown("bad band".to_owned()),
        BspectorError::Core(BsuiteCoreError::PromptResolution("bad prompt".to_owned())),
    ] {
        assert_error_exit_code(error, ExitCode::InternalError);
    }
}

#[test]
fn error_display_messages_embed_the_problematic_value() {
    let cases = [
        (
            BspectorError::ArtefactInputMalformed("bad-artefact".to_owned()),
            "bad-artefact",
        ),
        (
            BspectorError::TaxonomyUnknown("bad-pattern".to_owned()),
            "bad-pattern",
        ),
        (
            BspectorError::ScanStateUnknown("bad-state".to_owned()),
            "bad-state",
        ),
        (
            BspectorError::ArtefactShapeUnknown("bad-shape".to_owned()),
            "bad-shape",
        ),
        (
            BspectorError::StrictnessUnknown("bad-strictness".to_owned()),
            "bad-strictness",
        ),
        (
            BspectorError::RiskScoreBandUnknown("bad-band".to_owned()),
            "bad-band",
        ),
        (BspectorError::Usage("bad-args".to_owned()), "bad-args"),
    ];
    for (error, expected_fragment) in cases {
        assert!(
            error.to_string().contains(expected_fragment),
            "error Display missing fragment {expected_fragment:?}: {error}"
        );
    }
}

#[test]
fn process_exit_code_converts_all_exit_code_variants() {
    for (bsuite_code, expected_raw) in [
        (ExitCode::Success, 0u8),
        (ExitCode::Finding, 1u8),
        (ExitCode::InternalError, 2u8),
        (ExitCode::Usage, 64u8),
    ] {
        assert_eq!(
            std::process::ExitCode::from(expected_raw),
            process_exit_code(bsuite_code),
            "ExitCode::{bsuite_code:?} must produce raw exit code {expected_raw}",
        );
    }
}
