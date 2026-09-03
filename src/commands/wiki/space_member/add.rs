use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc wiki space-member add` 的参数。
#[derive(Debug, Args)]
pub struct AddArgs {
    /// Space id
    #[arg(value_name = "SPACE_ID")]
    pub space_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向空间中添加一个成员：`POST /v1/wiki/spaces/{space_id}/members`
/// （scope: `pcp:write:wiki:space`）。
///
/// 请求体必填 `member`（成员引用，如 `{"type":"user","id":"<user_id>"}`
/// 或团队引用），可选 `role_id`（空间角色 id），完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postWikiSpacesBySpaceIdMembers
pub async fn run(ctx: &Ctx, args: &AddArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/wiki/spaces/{}/members", args.space_id);
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
