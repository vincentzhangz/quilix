mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "quilix")]
#[command(version = "0.1.0")]
#[command(disable_version_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateApp {
        name: String,
        #[arg(long)]
        typescript: bool,
        #[arg(long)]
        tailwind: bool,
        #[arg(long)]
        biome: bool,
        #[arg(long)]
        app_router: bool,
        #[arg(long)]
        src_dir: bool,
        #[arg(long)]
        module_fed: bool,
        #[arg(long)]
        example: bool,
    },
    Dev {
        #[arg(long, default_value = "3000")]
        port: u16,
    },
    Build {
        #[arg(long)]
        target: Option<String>,
    },
    Preview {
        #[arg(long, default_value = "3000")]
        port: u16,
    },
    Lint {
        #[arg(long)]
        fix: bool,
        #[arg(long)]
        dir: Option<String>,
    },
    Generate {
        #[command(subcommand)]
        generator: Generator,
    },
    Add {
        plugin: String,
        #[arg(long)]
        host: bool,
        #[arg(long)]
        remote: bool,
    },
}

#[derive(Subcommand)]
enum Generator {
    Page { name: String },
    Component { name: String },
    Api { name: String },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::CreateApp { name, .. } => commands::create_app(&name),
        Commands::Dev { port } => commands::dev(port),
        Commands::Build { target } => commands::build(target),
        Commands::Preview { port } => commands::preview(port),
        Commands::Lint { fix, dir } => commands::lint(fix, dir),
        Commands::Generate { generator } => match generator {
            Generator::Page { name } => commands::generate_page(&name),
            Generator::Component { name } => commands::generate_component(&name),
            Generator::Api { name } => commands::generate_api(&name),
        },
        Commands::Add {
            plugin,
            host,
            remote,
        } => commands::add(&plugin, host, remote),
    }
}
