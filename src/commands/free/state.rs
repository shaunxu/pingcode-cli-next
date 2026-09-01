use anyhow::Result;
use serde_json::{json, Value};

use crate::client::{ClientError, Team, User};
use crate::commands::Ctx;
use crate::output;

/// `pc state` — 展示认证状态、企业（团队）与当前用户信息。
pub async fn run(ctx: &Ctx) -> Result<()> {
    let config = &ctx.config;

    if config.dry_run {
        // dry-run：只预览会发出的请求，不接触网络。
        // /v1/directory/team 与 /v1/myself 的请求预览由 client 打印。
        let _team: Value = ctx.client.get("/v1/directory/team").await?;
        let _user: Value = ctx.client.get("/v1/myself").await?;
        eprintln!("[dry-run] no response to display (requests were not sent)");
        return Ok(());
    }

    let team: Team = ctx.client.get("/v1/directory/team").await.map_err(|err| {
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

    // /v1/myself 仅用户令牌可访问；企业令牌不关联用户，此调用预期返回错误。
    let user: Option<User> = ctx.client.get::<User>("/v1/myself").await.ok();

    if config.json {
        let payload = json!({
            "authenticated": true,
            "authentication_method": config.credentials.label(),
            "base_url": config.base_url,
            "team": team,
            "user": user,
        });
        output::print_json(&payload)?;
        return Ok(());
    }

    println!("Authentication status: authenticated");
    println!("Authentication method: {}", config.credentials.label());
    println!("API base URL:       {}", config.base_url);

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
    match &user {
        Some(user) => {
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
        None => {
            println!("User: none (enterprise token is not associated with a user)");
        }
    }

    Ok(())
}
