use anyhow::Result;
use handlebars::Handlebars;
use std::collections::HashMap;
use walkdir::WalkDir;

/// Creates a new Quilix application from the basic template.
pub fn create_app(name: &str) -> Result<()> {
    tracing::info!("Creating new Quilix app: {}", name);
    println!("Creating app: {}", name);

    let template_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("packages")
        .join("@quilix")
        .join("create-app")
        .join("templates")
        .join("basic");

    let target_dir = std::env::current_dir()?.join(name);

    if target_dir.exists() {
        println!("Error: Directory {} already exists", name);
        return Ok(());
    }

    std::fs::create_dir_all(&target_dir)?;

    let hb = Handlebars::new();

    let mut data = HashMap::new();
    data.insert("name".to_string(), name.to_string());

    copy_template_dir(&template_dir, &target_dir, &hb, &data)?;

    println!("Created {} successfully!", name);
    println!("\ncd {}", name);
    println!("pnpm install");
    println!("pnpm dev");

    Ok(())
}

fn copy_template_dir(
    src: &std::path::Path,
    dest: &std::path::Path,
    hb: &Handlebars,
    data: &HashMap<String, String>,
) -> Result<()> {
    for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
        let relative = entry.path().strip_prefix(src)?;
        let target = dest.join(relative);

        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&target)?;
        } else if let Some(ext) = entry.path().extension() {
            if ext == "json" || ext == "ts" || ext == "tsx" || ext == "js" {
                let content = std::fs::read_to_string(entry.path())?;
                let rendered = hb.render_template(&content, data).unwrap_or(content);
                std::fs::write(&target, rendered)?;
            } else {
                std::fs::copy(entry.path(), &target)?;
            }
        }
    }
    Ok(())
}
