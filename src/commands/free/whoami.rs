use anyhow::Result;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc whoami` — 占位实现：具体端点待对照 PingCode Open API 文档确认。
///
/// 当前以 JSON 透传方式打印当前凭据对应的用户信息。
pub async fn run(ctx: &Ctx) -> Result<()> {
    let config = &ctx.config;

    if config.verbose && !config.dry_run {
        eprintln!("base_url = {}", config.base_url);
    }

    // 占位端点，响应按 serde_json::Value 透传，不假设字段结构。
    let user: Value = ctx.client.get("/v1/user").await?;

    if config.dry_run {
        // dry-run 下没有真实响应，仅提示请求已预览。
        eprintln!("[dry-run] no response to display (request was not sent)");
        return Ok(());
    }

    output::print_json(&user)?;
    Ok(())
}
