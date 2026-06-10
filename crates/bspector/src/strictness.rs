use crate::BspectorError;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Strictness {
    Lax,
    Default,
    Strict,
    Sealed,
}

impl Strictness {
    pub const ALL: [Self; 4] = [Self::Lax, Self::Default, Self::Strict, Self::Sealed];

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::Lax => "lax",
            Self::Default => "default",
            Self::Strict => "strict",
            Self::Sealed => "sealed",
        }
    }
}

impl fmt::Display for Strictness {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.stable_name())
    }
}

impl FromStr for Strictness {
    type Err = BspectorError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "lax" => Ok(Self::Lax),
            "default" => Ok(Self::Default),
            "strict" => Ok(Self::Strict),
            "sealed" => Ok(Self::Sealed),
            _ => Err(BspectorError::StrictnessUnknown(value.to_owned())),
        }
    }
}
