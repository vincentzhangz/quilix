use std::path::{Path, PathBuf};

/// Finds the project root by walking up for quilix.config.ts, package.json, or Cargo.toml.
pub fn find_project_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join("quilix.config.ts").exists()
            || current.join("package.json").exists()
            || current.join("Cargo.toml").exists()
        {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

/// Normalizes a path to absolute if relative.
pub fn normalize_path(path: &Path) -> PathBuf {
    if path.is_relative() {
        std::env::current_dir().unwrap_or_default().join(path)
    } else {
        path.to_path_buf()
    }
}
