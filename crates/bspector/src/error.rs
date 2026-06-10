use thiserror::Error;

#[derive(Debug, Error)]
pub enum BspectorError {
    #[error("artefact input is malformed: {0}")]
    ArtefactInputMalformed(String),
    #[error("unknown vulnerability pattern: {0}")]
    TaxonomyUnknown(String),
    #[error("unknown scan state: {0}")]
    ScanStateUnknown(String),
    #[error("unknown artefact shape: {0}")]
    ArtefactShapeUnknown(String),
    #[error("unknown strictness level: {0}")]
    StrictnessUnknown(String),
    #[error("unknown risk score band: {0}")]
    RiskScoreBandUnknown(String),
    #[error("argument usage is invalid: {0}")]
    Usage(String),
    #[error(transparent)]
    Core(#[from] bsuite_core::BsuiteCoreError),
}

impl BspectorError {
    pub const fn exit_code(&self) -> bsuite_core::ExitCode {
        match self {
            Self::Usage(_) => bsuite_core::ExitCode::Usage,
            Self::ArtefactInputMalformed(_)
            | Self::TaxonomyUnknown(_)
            | Self::ScanStateUnknown(_)
            | Self::ArtefactShapeUnknown(_)
            | Self::StrictnessUnknown(_)
            | Self::RiskScoreBandUnknown(_)
            | Self::Core(_) => bsuite_core::ExitCode::InternalError,
        }
    }

    pub fn process_exit_code(&self) -> std::process::ExitCode {
        process_exit_code(self.exit_code())
    }
}

pub fn process_exit_code(code: bsuite_core::ExitCode) -> std::process::ExitCode {
    std::process::ExitCode::from(code.as_i32() as u8)
}
