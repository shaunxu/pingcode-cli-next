use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization group create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个团队：`POST /v1/directory/user_groups`
/// （scope: `pcp:write:global:team`）。
///
/// 请求体必填 `name`（企业内唯一）；可选 `visibility`（`private` / `public`，
/// 默认 `private`）、`description`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postDirectoryUserGroups
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/directory/user_groups", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
