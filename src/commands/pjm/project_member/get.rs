use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project-member get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Member id (enterprise user id or team id)
    #[arg(value_name = "MEMBER_ID")]
    pub member_id: String,
}

/// 获取项目中的一个成员：`GET /v1/pjm/projects/{project_id}/members/{member_id}`
/// （scope: `pcp:read:pjm:project`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdMembersByMemberId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/members/{}",
        args.project_id, args.member_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
