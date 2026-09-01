mod cli;
mod client;
mod commands;
mod config;

use clap::Parser;

use crate::cli::Cli;
use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 自动加载工作目录下的 .env；已设置的真实环境变量优先，不会被 .env 覆盖
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();
    let config = Config::from_cli(&cli)?;

    if cli.verbose {
        eprintln!("verbose mode enabled");
    }

    commands::run(cli.command, &config).await
}
