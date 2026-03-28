use anyhow::Result;

/// Runs Biome lint on the project.
pub fn lint(fix: bool, _dir: Option<String>) -> Result<()> {
    tracing::info!("Running biome lint (fix={})", fix);

    let root_dir = std::env::current_dir()?;
    let target = if fix { "check --write" } else { "check" };

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let output = tokio::process::Command::new("npx")
            .args(["@biomejs/biome", target])
            .current_dir(&root_dir)
            .output()
            .await?;

        if output.status.success() {
            if fix {
                println!("Biome fixed issues successfully!");
            } else {
                println!("Biome check passed!");
            }
        } else {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
        Ok::<(), anyhow::Error>(())
    })?;

    Ok(())
}
