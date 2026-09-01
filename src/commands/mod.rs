use anyhow::Result;
use serde_json::Value;

use crate::cli::Command;
use crate::client::{ClientError, PingCodeClient, Team, User};
use crate::config::Config;

pub async fn run(command: Command, config: &Config) -> Result<()> {
    let client = PingCodeClient::new(config).await?;

    match command {
        Command::Whoami => whoami(&client, config).await,
        Command::State => state(&client, config).await,
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

async fn state(client: &PingCodeClient, config: &Config) -> Result<()> {
    println!("Authentication status: authenticated");
    println!("Authentication method: {}", config.credentials.label());
    println!("API base URL:       {}", config.base_url);

    let team: Team = client.get("/v1/directory/team").await.map_err(|err| {
        if let ClientError::Api { status, .. } = &err {
            if *status == 401 || *status == 403 {
                eprintln!("Authentication failed: invalid credentials or insufficient permissions to fetch team info.");
                if config.verbose {
                    eprintln!("Details: {err}");
                }
            }
        }
        err
    })?;

    println!();
    println!("Team:");
    println!("  Name: {}", team.name.as_deref().unwrap_or("-"));
    println!("  ID:   {}", team.id.as_deref().unwrap_or("-"));
    if let Some(domain) = team.secondary_domain.as_deref() {
        println!("  Secondary domain: {domain}");
    }
    if let Some(url) = team.url.as_deref() {
        println!("  URL:  {url}");
    }

    println!();
    // /v1/myself 仅用户令牌可访问；企业令牌不关联用户，此调用预期返回错误。
    match client.get::<User>("/v1/myself").await {
        Ok(user) => {
            println!("User:");
            println!(
                "  Name: {}",
                user.display_name
                    .as_deref()
                    .or(user.name.as_deref())
                    .unwrap_or("-")
            );
            println!("  ID:   {}", user.id.as_deref().unwrap_or("-"));
            if let Some(email) = user.email.as_deref() {
                println!("  Email: {email}");
            }
            if let Some(mobile) = user.mobile.as_deref() {
                println!("  Mobile: {mobile}");
            }
            if let Some(status) = user.status.as_deref() {
                println!("  Status: {status}");
            }
        }
        Err(_) => {
            println!("User: none (enterprise token is not associated with a user)");
        }
    }

    Ok(())
}
