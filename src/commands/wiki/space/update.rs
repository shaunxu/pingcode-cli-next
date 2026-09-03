use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc wiki space update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Space id
    #[arg(value_name = "SPACE_ID")]
    pub space_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个空间：`PATCH /v1/wiki/spaces/{space_id}`
/// （scope: `pcp:write:wiki:space`）。
///
/// 请求体可选 `name`、`identifier`、`description`。
/// 企业令牌不能更新 `scope_type` 为 `user` 的空间。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchWikiSpacesBySpaceId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/wiki/spaces/{}", args.space_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
