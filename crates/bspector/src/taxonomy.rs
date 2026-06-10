use crate::BspectorError;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VulnerabilityPattern {
    PromptInjection,
    DataExfiltration,
    PrivilegeEscalation,
    SupplyChain,
    ExcessiveAgency,
    OutputHandling,
    SystemPromptLeakage,
    MemoryPoisoning,
    ToolMisuse,
    RogueAgent,
    TriggerAbuse,
    LeastPrivilegeViolation,
}

impl VulnerabilityPattern {
    pub const ALL: [Self; 12] = [
        Self::PromptInjection,
        Self::DataExfiltration,
        Self::PrivilegeEscalation,
        Self::SupplyChain,
        Self::ExcessiveAgency,
        Self::OutputHandling,
        Self::SystemPromptLeakage,
        Self::MemoryPoisoning,
        Self::ToolMisuse,
        Self::RogueAgent,
        Self::TriggerAbuse,
        Self::LeastPrivilegeViolation,
    ];

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::PromptInjection => "prompt-injection",
            Self::DataExfiltration => "data-exfiltration",
            Self::PrivilegeEscalation => "privilege-escalation",
            Self::SupplyChain => "supply-chain",
            Self::ExcessiveAgency => "excessive-agency",
            Self::OutputHandling => "output-handling",
            Self::SystemPromptLeakage => "system-prompt-leakage",
            Self::MemoryPoisoning => "memory-poisoning",
            Self::ToolMisuse => "tool-misuse",
            Self::RogueAgent => "rogue-agent",
            Self::TriggerAbuse => "trigger-abuse",
            Self::LeastPrivilegeViolation => "least-privilege-violation",
        }
    }
}

impl fmt::Display for VulnerabilityPattern {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.stable_name())
    }
}

impl FromStr for VulnerabilityPattern {
    type Err = BspectorError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "prompt-injection" => Ok(Self::PromptInjection),
            "data-exfiltration" => Ok(Self::DataExfiltration),
            "privilege-escalation" => Ok(Self::PrivilegeEscalation),
            "supply-chain" => Ok(Self::SupplyChain),
            "excessive-agency" => Ok(Self::ExcessiveAgency),
            "output-handling" => Ok(Self::OutputHandling),
            "system-prompt-leakage" => Ok(Self::SystemPromptLeakage),
            "memory-poisoning" => Ok(Self::MemoryPoisoning),
            "tool-misuse" => Ok(Self::ToolMisuse),
            "rogue-agent" => Ok(Self::RogueAgent),
            "trigger-abuse" => Ok(Self::TriggerAbuse),
            "least-privilege-violation" => Ok(Self::LeastPrivilegeViolation),
            _ => Err(BspectorError::TaxonomyUnknown(value.to_owned())),
        }
    }
}
