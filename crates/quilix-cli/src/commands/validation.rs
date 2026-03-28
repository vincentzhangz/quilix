use anyhow::Result;
use std::path::{Path, PathBuf};

/// Validates the current directory is a Quilix project.
pub fn quilix_project_check(root_dir: &Path) -> Result<()> {
    if !root_dir.join("app").exists()
        && !root_dir.join("quilix.config.ts").exists()
        && !root_dir.join("src").exists()
    {
        anyhow::bail!(
            "Not a quilix project directory. Run 'quilix dev' from within a quilix project (with app/, src/, or quilix.config.ts)"
        );
    }
    Ok(())
}

/// Checks that rspack.config.mjs exists in the project root.
pub fn check_rspack_config(root_dir: &Path) -> Result<PathBuf> {
    let rspack_config_path = root_dir.join("rspack.config.mjs");
    if !rspack_config_path.exists() {
        println!();
        println!("Error: rspack.config.mjs not found in current directory.");
        println!();
        println!("To get started, create a new Quilix app:");
        println!("  quilix create-app my-app");
        println!();
        println!("Or create a rspack.config.mjs manually. See: https://rspack.dev/");
        println!();
        anyhow::bail!("rspack.config.mjs is required");
    }
    Ok(rspack_config_path)
}

/// Gets the project name from package.json.
fn get_project_name(root_dir: &Path) -> String {
    let package_json_path = root_dir.join("package.json");
    if let Ok(content) = std::fs::read_to_string(&package_json_path)
        && let Ok(json) = serde_json::from_str::<serde_json::Value>(&content)
        && let Some(name) = json.get("name").and_then(|n| n.as_str())
    {
        return name.split('/').next_back().unwrap_or(name).to_string();
    }
    "Quilix App".to_string()
}

/// Ensures public/index.html exists, creating a default if missing.
pub fn ensure_public_index(root_dir: &Path) -> Result<()> {
    let public_dir = root_dir.join("public");
    let index_path = public_dir.join("index.html");

    if !index_path.exists() {
        std::fs::create_dir_all(&public_dir)?;
        let title = get_project_name(root_dir);
        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{}</title>
</head>
<body>
    <div id="root"></div>
</body>
</html>
"#,
            title
        );
        std::fs::write(&index_path, html)?;
    }
    Ok(())
}
