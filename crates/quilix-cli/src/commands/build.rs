use anyhow::{Context, Result};
use std::path::Path;
use walkdir::WalkDir;

use crate::commands::validation::{check_rspack_config, ensure_public_index, quilix_project_check};

/// Builds the Quilix application for production.
pub fn build(target: Option<String>) -> Result<()> {
    tracing::info!("Building for target: {:?}", target);
    println!("Building for production...");

    let root_dir = std::env::current_dir()?;

    quilix_project_check(&root_dir)?;
    ensure_public_index(&root_dir)?;
    check_rspack_config(&root_dir)?;

    spawn_rspack_build(&root_dir)?;

    println!("Build completed successfully!");

    Ok(())
}

fn spawn_rspack_build(root_dir: &Path) -> Result<()> {
    let rspack_path = find_rspack_binary(root_dir)?;

    let status = std::process::Command::new("node")
        .arg(&rspack_path)
        .arg("build")
        .arg("--config")
        .arg("rspack.config.mjs")
        .current_dir(root_dir)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .context("Failed to spawn rspack build")?;

    if !status.success() {
        anyhow::bail!("rspack build exited with status: {}", status);
    }

    Ok(())
}

fn find_rspack_binary(root_dir: &Path) -> Result<String> {
    let rspack_path = root_dir.join("node_modules/@rspack/cli/bin/rspack.js");

    if rspack_path.exists() {
        return Ok(rspack_path.to_string_lossy().to_string());
    }

    let workspace_root = find_workspace_root(root_dir);
    if let Some(root) = workspace_root {
        let pnpm_store_path = root.join("node_modules/.pnpm");
        if pnpm_store_path.exists() {
            let rspack_path = find_in_pnpm_store(&pnpm_store_path)?;
            if let Some(path) = rspack_path {
                return Ok(path);
            }
        }
    }

    let resolved = resolve_via_node(root_dir, "@rspack/cli/bin/rspack.js")?;
    if let Some(path) = resolved {
        return Ok(path);
    }

    anyhow::bail!("Could not find @rspack/core. Run 'pnpm install' first.")
}

fn find_workspace_root(start: &Path) -> Option<std::path::PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        let pkg = current.join("package.json");
        if pkg.exists() {
            let content = std::fs::read_to_string(&pkg).ok()?;
            if content.contains("\"workspaces\"") {
                return Some(current);
            }
        }
        if !current.pop() {
            return None;
        }
    }
}

fn find_in_pnpm_store(pnpm_dir: &Path) -> Result<Option<String>> {
    for entry in WalkDir::new(pnpm_dir)
        .max_depth(8)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.ends_with("rspack.js") && path.to_string_lossy().contains("@rspack/cli/bin") {
            return Ok(Some(path.to_string_lossy().to_string()));
        }
    }
    Ok(None)
}

fn resolve_via_node(root_dir: &Path, module: &str) -> Result<Option<String>> {
    let script = format!("console.log(require.resolve('{}'))", module);
    let output = std::process::Command::new("node")
        .arg("-e")
        .arg(&script)
        .current_dir(root_dir)
        .output()?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !path.is_empty() && path != "undefined" && !path.contains("Cannot find") {
            return Ok(Some(path));
        }
    }

    Ok(None)
}
