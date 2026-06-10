use crate::BspectorError;
use std::path::PathBuf;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SubstrateInput {
    pub artefact: String,
    pub artefact_shape: Option<String>,
    pub strictness: Option<String>,
    pub llm_stage: bool,
    pub osv_online: bool,
    pub manifest: Option<PathBuf>,
    pub reason: Option<String>,
}

impl SubstrateInput {
    pub fn new(
        artefact: String,
        artefact_shape: Option<String>,
        strictness: Option<String>,
        llm_stage: bool,
        osv_online: bool,
        manifest: Option<PathBuf>,
        reason: Option<String>,
    ) -> Result<Self, BspectorError> {
        if artefact.trim().is_empty() {
            return Err(BspectorError::ArtefactInputMalformed(
                "artefact must not be empty".to_owned(),
            ));
        }

        Ok(Self {
            artefact,
            artefact_shape,
            strictness,
            llm_stage,
            osv_online,
            manifest,
            reason,
        })
    }
}
