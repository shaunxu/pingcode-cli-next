mod cli;
mod client;
mod commands;
mod config;

use clap::Parser;

use crate::cli::Cli;
use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = Config::from_cli(&cli)?;

    if cli.verbose {
        eprintln!("verbose 模式已开启");
    }

    commands::run(cli.command, &config).await
}
