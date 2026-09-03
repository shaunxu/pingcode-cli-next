use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization group-member get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Team/group id
    #[arg(value_name = "GROUP_ID")]
    pub group_id: String,

    /// Member id (the enterprise user id)
    #[arg(value_name = "MEMBER_ID")]
    pub member_id: String,
}

/// 获取团队中的一个成员：`GET /v1/directory/groups/{group_id}/members/{member_id}`
/// （scope: `pcp:read:global:team`）。
///
/// 返回成员引用（user）、所属团队引用（group）及在团队中的角色（role）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryGroupsByGroupIdMembersByMemberId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/directory/groups/{}/members/{}",
        args.group_id, args.member_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
