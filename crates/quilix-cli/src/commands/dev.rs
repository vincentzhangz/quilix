use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::io::AsyncBufReadExt;
use tokio::process::Command;

use crate::commands::validation::{check_rspack_config, ensure_public_index, quilix_project_check};

/// Starts the Quilix development server.
pub fn dev(port: u16) -> Result<()> {
    tracing::info!("Starting Quilix dev server on port {}", port);

    let root_dir = std::env::current_dir()?;

    quilix_project_check(&root_dir)?;
    ensure_public_index(&root_dir)?;

    let rspack_config_path = check_rspack_config(&root_dir)?;

    let dist_dir = root_dir.join("dist");
    std::fs::create_dir_all(&dist_dir)?;

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(spawn_rspack_dev(&root_dir, &rspack_config_path, port))?;

    Ok(())
}

async fn spawn_rspack_dev(root_dir: &Path, config_path: &Path, port: u16) -> Result<()> {
    let rspack_path = find_rspack_binary(root_dir)?;

    println!();
    println!("Quilix v0.1.0");
    println!("- Local:        http://localhost:{}", port);
    println!();
    println!("⇢ Compiling...");

    let node_modules_path = get_node_modules_path(root_dir);
    let mut cmd = Command::new("node");
    cmd.arg(&rspack_path)
        .arg("dev")
        .arg("--config")
        .arg(config_path)
        .current_dir(root_dir);

    if !node_modules_path.is_empty() {
        cmd.env("NODE_PATH", node_modules_path);
    }

    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn rspack dev server")?;

    let stderr = child.stderr.take().unwrap();
    tokio::spawn(async move {
        let reader = tokio::io::BufReader::new(stderr);
        let mut lines = reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if line.starts_with("<i> [webpack-dev-server]") {
                continue;
            }
            if line.starts_with("ERROR") || line.contains("Module not found") {
                eprintln!("✗ {}", line);
            }
        }
    });

    let stdout = child.stdout.take().unwrap();
    let mut reader = tokio::io::BufReader::new(stdout).lines();
    let mut was_ready = false;

    while let Ok(Some(line)) = reader.next_line().await {
        if line.starts_with("<i> [webpack-dev-server]") {
            continue;
        }
        if line.starts_with("ERROR") || line.contains("Module not found") {
            eprintln!("✗ {}", line);
            was_ready = false;
            continue;
        }
        if line.contains("compiled successfully") || line.contains("Rspack compiled") {
            if was_ready {
                println!("⇢ Compiling...");
            }
            let timing = if line.contains(" in ") {
                line.split(" in ")
                    .nth(1)
                    .and_then(|s| s.trim().split(' ').next())
                    .map(|t| format!(" in {} ms", t))
                    .unwrap_or_default()
            } else {
                String::new()
            };
            println!("✓ Ready{}", timing);
            was_ready = true;
        }
    }

    let status = child.wait().await?;

    if !status.success() {
        anyhow::bail!("rspack dev server exited with status: {}", status);
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
        let rspack_path = root.join("node_modules/@rspack/cli/bin/rspack.js");
        if rspack_path.exists() {
            return Ok(rspack_path.to_string_lossy().to_string());
        }

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

fn get_node_modules_path(root_dir: &Path) -> String {
    let nm = root_dir.join("node_modules");
    if nm.exists() {
        return nm.to_string_lossy().to_string();
    }

    if let Some(root) = find_workspace_root(root_dir) {
        let nm = root.join("node_modules");
        if nm.exists() {
            return nm.to_string_lossy().to_string();
        }
    }

    String::new()
}

fn find_workspace_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join("pnpm-workspace.yaml").exists() || current.join("package.json").exists() {
            let pkg = current.join("package.json");
            if pkg.exists() {
                let content = std::fs::read_to_string(&pkg).ok()?;
                if content.contains("\"workspaces\"") {
                    return Some(current);
                }
            }
        }
        if !current.pop() {
            return None;
        }
    }
}

fn find_in_pnpm_store(pnpm_dir: &Path) -> Result<Option<String>> {
    for entry in walkdir::WalkDir::new(pnpm_dir)
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
