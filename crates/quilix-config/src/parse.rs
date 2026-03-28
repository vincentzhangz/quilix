use crate::config::{MfMode, ModuleFederationConfig, PluginConfig, QuilixConfig};
use anyhow::Result;
use regex::Regex;

/// Parses a quilix.config.ts content string into a QuilixConfig.
pub fn parse_config(content: &str) -> Result<QuilixConfig> {
    let mut config = QuilixConfig::default();

    let plugins_re = Regex::new(r"plugins:\s*\[\s*(\{[^\]]*\})\s*\]").unwrap();
    if let Some(caps) = plugins_re.captures(content) {
        let plugins_str = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let name_re = Regex::new(r"name:\s*(\w+)")?;
        for plugin_match in name_re.find_iter(plugins_str) {
            if let Some(name_match) = name_re.captures(plugin_match.as_str()) {
                config.plugins.push(PluginConfig {
                    name: name_match
                        .get(1)
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default(),
                    options: serde_json::Value::Null,
                });
            }
        }
    }

    let mf_re = Regex::new(r"moduleFederation\s*\(\s*\{([^}]+)\}\s*\)").unwrap();
    if let Some(caps) = mf_re.captures(content) {
        let mf_str = caps.get(1).map(|m| m.as_str()).unwrap_or("");

        let name_re = Regex::new(r#"name:\s*["'](\w+)["']"#)?;
        let name = name_re
            .captures(mf_str)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();

        let mode_re = Regex::new(r#"mode:\s*["'](\w+)["']"#)?;
        let mode = mode_re
            .captures(mf_str)
            .and_then(|c| c.get(1))
            .map(|m| match m.as_str() {
                "host" => MfMode::Host,
                "remote" => MfMode::Remote,
                "standalone" => MfMode::Standalone,
                _ => MfMode::Host,
            })
            .unwrap_or(MfMode::Host);

        let mut remotes = std::collections::HashMap::new();
        let remotes_re = Regex::new(r#"(\w+):\s*["']([^"']+)["']"#).unwrap();
        for caps in remotes_re.captures_iter(mf_str) {
            if let (Some(key), Some(val)) = (caps.get(1), caps.get(2)) {
                let key_str = key.as_str();
                if key_str != "name"
                    && key_str != "mode"
                    && key_str != "shared"
                    && key_str != "exposes"
                {
                    remotes.insert(key_str.to_string(), val.as_str().to_string());
                }
            }
        }

        let mut shared = Vec::new();
        let shared_re = Regex::new(r#"shared:\s*\[\s*([^]]+)\]"#).unwrap();
        let shared_item_re = Regex::new(r#"["'](\w+)["']"#)?;
        if let Some(shared_caps) = shared_re.captures(mf_str)
            && let Some(shared_str) = shared_caps.get(1)
        {
            for item in shared_item_re.find_iter(shared_str.as_str()) {
                shared.push(
                    item.as_str()
                        .trim_matches('"')
                        .trim_matches('\'')
                        .to_string(),
                );
            }
        }

        let mut exposes = std::collections::HashMap::new();
        let exposes_re = Regex::new(r#"exposes:\s*\{([^}]+)\}"#).unwrap();
        if let Some(exposes_caps) = exposes_re.captures(mf_str)
            && let Some(exposes_str) = exposes_caps.get(1)
        {
            let exposes_items_re = Regex::new(r#"["']([^"']+)["']:\s*["']([^"']+)["']"#).unwrap();
            for item in exposes_items_re.captures_iter(exposes_str.as_str()) {
                if let (Some(key), Some(val)) = (item.get(1), item.get(2)) {
                    exposes.insert(key.as_str().to_string(), val.as_str().to_string());
                }
            }
        }

        if !name.is_empty() || !remotes.is_empty() || !shared.is_empty() {
            config.module_federation = Some(ModuleFederationConfig {
                name,
                mode,
                remotes,
                shared,
                exposes,
            });
        }
    }

    Ok(config)
}
