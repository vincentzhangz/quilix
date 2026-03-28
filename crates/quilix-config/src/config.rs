use serde::{Deserialize, Serialize};

/// Top-level Quilix configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuilixConfig {
    #[serde(default)]
    pub plugins: Vec<PluginConfig>,
    #[serde(default)]
    pub module_federation: Option<ModuleFederationConfig>,
}

/// Plugin configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginConfig {
    pub name: String,
    #[serde(default)]
    pub options: serde_json::Value,
}

/// Module Federation configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModuleFederationConfig {
    pub name: String,
    #[serde(rename = "mode", default)]
    pub mode: MfMode,
    #[serde(default)]
    pub remotes: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub shared: Vec<String>,
    #[serde(default)]
    pub exposes: std::collections::HashMap<String, String>,
}

/// Module Federation deployment mode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum MfMode {
    #[default]
    Host,
    Remote,
    #[serde(rename = "standalone")]
    Standalone,
}
