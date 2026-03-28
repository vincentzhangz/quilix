use anyhow::Result;
use std::fs;

/// Reads a file into a string.
pub fn read_file(path: &std::path::Path) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

/// Writes content to a file, creating parent directories if needed.
pub fn write_file(path: &std::path::Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}

/// Recursively copies a directory.
pub fn copy_dir_recursive(from: &std::path::Path, to: &std::path::Path) -> Result<()> {
    fs::create_dir_all(to)?;
    for entry in walkdir::WalkDir::new(from) {
        let entry = entry?;
        let target = to.join(entry.path().strip_prefix(from)?);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}
