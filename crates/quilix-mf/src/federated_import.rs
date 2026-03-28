use anyhow::Result;
use regex::Regex;

/// Transforms federated import syntax (fm:remote/module) to standard imports.
pub fn transform_federated_imports(code: &str) -> Result<String> {
    let fm_import_re = Regex::new(r#"import\s*\(\s*['"]fm:([^'"]+)/([^'"]+)['"]\s*\)"#)?;

    let transformed = fm_import_re.replace_all(code, |caps: &regex::Captures| {
        let remote = &caps[1];
        let _module = &caps[2];
        format!("import('{}')", remote)
    });

    Ok(transformed.into_owned())
}

/// Parses a federated URL into (remote, module) tuple.
pub fn parse_federated_url(url: &str) -> Option<(String, String)> {
    if let Some(stripped) = url.strip_prefix("fm:") {
        let parts = stripped.split('/').collect::<Vec<_>>();
        if parts.len() >= 2 {
            let remote = parts[0].to_string();
            let module = parts[1..].join("/");
            return Some((remote, module));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_federated_import() {
        let code = r#"const Module = await import('fm:shop/Button');"#;
        let result = transform_federated_imports(code).unwrap();
        assert_eq!(result, "const Module = await import('shop');");
    }

    #[test]
    fn test_parse_federated_url() {
        assert_eq!(
            parse_federated_url("fm:shop/Button"),
            Some(("shop".to_string(), "Button".to_string()))
        );
        assert_eq!(
            parse_federated_url("fm:remote/path/to/module"),
            Some(("remote".to_string(), "path/to/module".to_string()))
        );
        assert_eq!(parse_federated_url("normal/path"), None);
    }
}
