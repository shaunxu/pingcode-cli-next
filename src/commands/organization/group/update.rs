use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization group update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Team/group id
    #[arg(value_name = "GROUP_ID")]
    pub group_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个团队：`PATCH /v1/directory/groups/{group_id}`
/// （scope: `pcp:write:global:team`）。
///
/// 请求体可选 `name`、`visibility`（`private` / `public`）、`description`。
/// 开放平台未提供删除团队的端点。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchDirectoryGroupsByGroupId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/directory/groups/{}", args.group_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
