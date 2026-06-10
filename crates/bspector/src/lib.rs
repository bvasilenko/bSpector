pub mod artefact_shape;
pub mod cli;
pub mod error;
pub mod risk_band;
pub mod scan;
pub mod scan_state;
pub mod strictness;
pub mod substrate_input;
pub mod taxonomy;

pub use artefact_shape::ArtefactShape;
pub use cli::{BspectorCli, Command, ScanArgs};
pub use error::BspectorError;
pub use risk_band::RiskScoreBand;
pub use scan_state::ScanState;
pub use strictness::Strictness;
pub use taxonomy::VulnerabilityPattern;

pub fn routing_key() -> bsuite_core::RoutingKey {
    bsuite_core::RoutingKey::bspector()
}
