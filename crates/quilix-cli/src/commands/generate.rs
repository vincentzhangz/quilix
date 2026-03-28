use anyhow::Result;

use crate::commands::templates::{get_api_template, get_component_template, get_page_template};

/// Generates a new page component.
pub fn generate_page(name: &str) -> Result<()> {
    tracing::info!("Generating page: {}", name);
    let root_dir = std::env::current_dir()?;
    let page_path = root_dir.join("app").join(name).join("page.tsx");

    if page_path.exists() {
        println!("Page already exists: {}", page_path.display());
        return Ok(());
    }

    std::fs::create_dir_all(page_path.parent().unwrap())?;
    std::fs::write(&page_path, get_page_template(name))?;
    println!("Created page: {}", page_path.display());

    Ok(())
}

/// Generates a new UI component.
pub fn generate_component(name: &str) -> Result<()> {
    tracing::info!("Generating component: {}", name);
    let root_dir = std::env::current_dir()?;
    let component_path = root_dir.join("components").join(format!(
        "{}.tsx",
        crate::commands::utils::to_pascal_case(name)
    ));

    if component_path.exists() {
        println!("Component already exists: {}", component_path.display());
        return Ok(());
    }

    if !component_path.parent().unwrap().exists() {
        std::fs::create_dir_all(component_path.parent().unwrap())?;
    }

    std::fs::write(&component_path, get_component_template(name))?;
    println!("Created component: {}", component_path.display());

    Ok(())
}

/// Generates a new API route.
pub fn generate_api(name: &str) -> Result<()> {
    tracing::info!("Generating API route: {}", name);
    let root_dir = std::env::current_dir()?;
    let api_path = root_dir.join("app").join("api").join(name).join("route.ts");

    if api_path.exists() {
        println!("API route already exists: {}", api_path.display());
        return Ok(());
    }

    std::fs::create_dir_all(api_path.parent().unwrap())?;
    std::fs::write(&api_path, get_api_template(name))?;
    println!("Created API route: {}", api_path.display());

    Ok(())
}
