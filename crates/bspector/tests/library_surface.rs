use bspector::{
    ArtefactShape, BspectorCli, BspectorError, Command, RiskScoreBand, ScanArgs, ScanState,
    Strictness, VulnerabilityPattern, routing_key,
};

struct PendingScanner;

impl PendingScanner {
    fn scan(&self) -> ScanState {
        unimplemented!("not yet implemented")
    }
}

#[test]
fn library_reexports_public_contract_types() {
    assert_eq!(12, VulnerabilityPattern::ALL.len());
    assert_eq!(4, ScanState::ALL.len());
    assert_eq!(7, ArtefactShape::ALL.len());
    assert_eq!(4, Strictness::ALL.len());
    assert_eq!(4, RiskScoreBand::ALL.len());
}

#[test]
fn library_reexports_cli_and_error_surface() {
    assert_ne!(0, std::mem::size_of::<BspectorCli>());
    assert_ne!(0, std::mem::size_of::<Command>());
    assert_ne!(0, std::mem::size_of::<ScanArgs>());
    assert_ne!(0, std::mem::size_of::<BspectorError>());
}

#[test]
fn routing_key_uses_bspector_core_entry_point() {
    assert_eq!(bsuite_core::RoutingKey::bspector(), routing_key());
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn placeholder_scanner_is_explicitly_pending() {
    let scanner = PendingScanner;
    let _ = scanner.scan();
}
