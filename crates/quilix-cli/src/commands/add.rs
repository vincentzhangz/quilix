use anyhow::Result;

/// Adds a plugin or integration to the project.
pub fn add(plugin: &str, host: bool, remote: bool) -> Result<()> {
    tracing::info!(
        "Adding plugin: {} (host={}, remote={})",
        plugin,
        host,
        remote
    );

    match plugin {
        "module-federation" => {
            println!("Adding Module Federation support...");
            add_module_federation(host, remote)?;
        }
        "tailwind" => {
            println!("Adding Tailwind CSS...");
            add_tailwind()?;
        }
        "shadcn" => {
            println!("Adding shadcn/ui...");
            add_shadcn()?;
        }
        _ => {
            println!(
                "Unknown plugin: {}. Available: module-federation, tailwind, shadcn",
                plugin
            );
        }
    }

    Ok(())
}

fn add_module_federation(host: bool, _remote: bool) -> Result<()> {
    let root_dir = std::env::current_dir()?;
    let config_path = root_dir.join("quilix.config.ts");

    let _mode = if host { "host" } else { "remote" };

    let config_content = format!(
        r#"import {{ defineConfig }} from '@quilix/core';
import {{ moduleFederation }} from '@quilix/core';

export default defineConfig({{
  plugins: [
    moduleFederation({{
      name: '{}',
      remotes: {{}},
      shared: ['react', 'react-dom'],
    }}),
  ],
}});
"#,
        if host { "host" } else { "remote" }
    );

    if config_path.exists() {
        println!("quilix.config.ts already exists, updating...");
    }

    std::fs::write(&config_path, config_content)?;
    println!("Added Module Federation ({}) to quilix.config.ts", _mode);

    Ok(())
}

fn add_tailwind() -> Result<()> {
    let root_dir = std::env::current_dir()?;

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let _ = tokio::process::Command::new("npx")
            .args(["tailwindcss", "init", "-p"])
            .current_dir(&root_dir)
            .output()
            .await?;
        Ok::<(), anyhow::Error>(())
    })?;

    let tailwind_config = r#"/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./app/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};"#;

    std::fs::write(root_dir.join("tailwind.config.js"), tailwind_config)?;
    println!("Added Tailwind CSS configuration");

    Ok(())
}

fn add_shadcn() -> Result<()> {
    let root_dir = std::env::current_dir()?;

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let _ = tokio::process::Command::new("npx")
            .args(["shadcn-ui@latest", "init", "-y"])
            .current_dir(&root_dir)
            .output()
            .await?;
        Ok::<(), anyhow::Error>(())
    })?;

    println!("Added shadcn/ui. Run 'npx shadcn-ui add <component>' to add components.");

    Ok(())
}
