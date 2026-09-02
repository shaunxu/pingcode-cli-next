use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project-member update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Member id (enterprise user id or team id)
    #[arg(value_name = "MEMBER_ID")]
    pub member_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新项目中的一个成员（角色）：
/// `PATCH /v1/pjm/projects/{project_id}/members/{member_id}`
/// （scope: `pcp:write:pjm:project`）。
///
/// 请求体字段 `role_id`（角色 id）；仅管理员可更改其他用户的角色。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdMembersByMemberId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/pjm/projects/{}/members/{}",
        args.project_id, args.member_id
    );
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
