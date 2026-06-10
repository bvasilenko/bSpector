use crate::BspectorError;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ArtefactShape {
    SkillMd,
    ManifestOverlayToml,
    PayloadPluginManifest,
    StrapiPluginManifest,
    SanityStudioActionManifest,
    DirectusExtensionManifest,
    GenericMcpManifest,
}

impl ArtefactShape {
    pub const ALL: [Self; 7] = [
        Self::SkillMd,
        Self::ManifestOverlayToml,
        Self::PayloadPluginManifest,
        Self::StrapiPluginManifest,
        Self::SanityStudioActionManifest,
        Self::DirectusExtensionManifest,
        Self::GenericMcpManifest,
    ];

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::SkillMd => "skill-md",
            Self::ManifestOverlayToml => "manifest-overlay-toml",
            Self::PayloadPluginManifest => "payload-plugin-manifest",
            Self::StrapiPluginManifest => "strapi-plugin-manifest",
            Self::SanityStudioActionManifest => "sanity-studio-action-manifest",
            Self::DirectusExtensionManifest => "directus-extension-manifest",
            Self::GenericMcpManifest => "generic-mcp-manifest",
        }
    }
}

impl fmt::Display for ArtefactShape {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.stable_name())
    }
}

impl FromStr for ArtefactShape {
    type Err = BspectorError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "skill-md" => Ok(Self::SkillMd),
            "manifest-overlay-toml" => Ok(Self::ManifestOverlayToml),
            "payload-plugin-manifest" => Ok(Self::PayloadPluginManifest),
            "strapi-plugin-manifest" => Ok(Self::StrapiPluginManifest),
            "sanity-studio-action-manifest" => Ok(Self::SanityStudioActionManifest),
            "directus-extension-manifest" => Ok(Self::DirectusExtensionManifest),
            "generic-mcp-manifest" => Ok(Self::GenericMcpManifest),
            _ => Err(BspectorError::ArtefactShapeUnknown(value.to_owned())),
        }
    }
}
