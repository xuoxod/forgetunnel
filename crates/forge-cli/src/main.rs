mod cli;
mod handlers;

use clap::Parser;
use cli::Cli;
use colored::Colorize;
use forge_core::ForgeConfig;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.generate_config {
        println!("{}", ForgeConfig::generate_default_template());
        return Ok(());
    }

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let (config, config_source) = ForgeConfig::discover_or_default(cli.config.as_deref())?;

    println!(
        "{}",
        "====================================================================".bright_blue()
    );
    println!(
        "{}",
        "⚡ FORGETUNNEL : SOVEREIGN LOCAL-AI & WEBHOOK GATEWAY (v0.1.0)"
            .bright_cyan()
            .bold()
    );
    println!(
        "   Node: {} | Config: {}",
        config.node_label.bright_yellow(),
        config_source.display().to_string().bright_green()
    );
    println!(
        "{}",
        "====================================================================".bright_blue()
    );

    handlers::dispatch_command(cli.command, config).await
}
