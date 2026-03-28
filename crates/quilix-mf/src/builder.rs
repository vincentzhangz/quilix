use anyhow::Result;
use quilix_config::{MfMode, ModuleFederationConfig};
use serde::{Deserialize, Serialize};

/// Rspack-compatible Module Federation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RspackModuleFederationConfig {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(
        rename = "remotes",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub remotes: std::collections::HashMap<String, String>,
    #[serde(rename = "shared", skip_serializing_if = "Vec::is_empty")]
    pub shared: Vec<SharedConfig>,
    #[serde(
        rename = "exposes",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub exposes: std::collections::HashMap<String, String>,
    #[serde(rename = "devtools", skip_serializing_if = "Option::is_none")]
    pub devtools: Option<String>,
}

/// Shared dependency configuration for Rspack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedConfig {
    #[serde(rename = "eager")]
    pub eager: Option<bool>,
    #[serde(rename = "requiredVersion")]
    pub required_version: Option<String>,
    #[serde(rename = "singleton")]
    pub singleton: Option<bool>,
    #[serde(rename = "strictVersion")]
    pub strict_version: Option<bool>,
    #[serde(rename = "packageType")]
    pub package_type: Option<String>,
    #[serde(rename = "shareKey")]
    pub share_key: Option<String>,
    #[serde(rename = "shareScope")]
    pub share_scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl From<&ModuleFederationConfig> for RspackModuleFederationConfig {
    fn from(mf: &ModuleFederationConfig) -> Self {
        let shared = mf
            .shared
            .iter()
            .map(|s| SharedConfig {
                eager: Some(false),
                required_version: Some(format!("^{}", s)),
                singleton: Some(true),
                strict_version: Some(false),
                package_type: None,
                share_key: Some(s.clone()),
                share_scope: Some("default".to_string()),
                version: None,
            })
            .collect();

        let devtools = if mf.mode == MfMode::Remote {
            Some("*".to_string())
        } else {
            None
        };

        Self {
            name: mf.name.clone(),
            remotes: mf.remotes.clone(),
            shared,
            exposes: mf.exposes.clone(),
            devtools,
        }
    }
}

/// Builds a Rspack Module Federation config from a Quilix config.
pub fn build_module_federation_config(
    mf_config: &ModuleFederationConfig,
) -> Result<RspackModuleFederationConfig> {
    Ok(RspackModuleFederationConfig::from(mf_config))
}

/// Returns the string representation of a Module Federation mode.
pub fn get_mf_mode_string(mode: &MfMode) -> &'static str {
    match mode {
        MfMode::Host => "host",
        MfMode::Remote => "remote",
        MfMode::Standalone => "standalone",
    }
}
