use anyhow::Result;
use serde_json::Value;

use crate::cli::Command;
use crate::client::PingCodeClient;
use crate::config::Config;

pub async fn run(command: Command, config: &Config) -> Result<()> {
    let client = PingCodeClient::new(config)?;

    match command {
        Command::Whoami => whoami(&client, config).await,
    }
}

async fn whoami(client: &PingCodeClient, config: &Config) -> Result<()> {
    // 占位实现：具体端点待对照 PingCode Open API 文档确认。
    // 当前以 JSON 透传方式打印当前凭据对应的用户信息。
    let user: Value = client.get("/v1/user").await?;

    if config.verbose {
        eprintln!("base_url = {}", config.base_url);
    }
    println!("{}", serde_json::to_string_pretty(&user)?);
    Ok(())
}
