use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization group-member add` 的参数。
#[derive(Debug, Args)]
pub struct AddArgs {
    /// Team/group id
    #[arg(value_name = "GROUP_ID")]
    pub group_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向团队中添加一个成员：`POST /v1/directory/groups/{group_id}/members`
/// （scope: `pcp:write:global:team`）。
///
/// 请求体必填 `user_id`（用户 id）与 `role`（团队角色，
/// `manager` 或 `member`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postDirectoryGroupsByGroupIdMembers
pub async fn run(ctx: &Ctx, args: &AddArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/directory/groups/{}/members", args.group_id);
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
