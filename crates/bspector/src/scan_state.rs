use crate::BspectorError;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScanState {
    Safe,
    Unsafe,
    Malformed,
    InternalError,
}

impl ScanState {
    pub const ALL: [Self; 4] = [
        Self::Safe,
        Self::Unsafe,
        Self::Malformed,
        Self::InternalError,
    ];

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::Unsafe => "unsafe",
            Self::Malformed => "malformed",
            Self::InternalError => "internal-error",
        }
    }
}

impl fmt::Display for ScanState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.stable_name())
    }
}

impl FromStr for ScanState {
    type Err = BspectorError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "safe" => Ok(Self::Safe),
            "unsafe" => Ok(Self::Unsafe),
            "malformed" => Ok(Self::Malformed),
            "internal-error" => Ok(Self::InternalError),
            _ => Err(BspectorError::ScanStateUnknown(value.to_owned())),
        }
    }
}

impl From<ScanState> for bsuite_core::ExitCode {
    fn from(value: ScanState) -> Self {
        match value {
            ScanState::Safe => Self::Success,
            ScanState::Unsafe => Self::Finding,
            ScanState::Malformed => Self::Usage,
            ScanState::InternalError => Self::InternalError,
        }
    }
}
