use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization group-member remove` 的参数。
#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Team/group id
    #[arg(value_name = "GROUP_ID")]
    pub group_id: String,

    /// Member id (the enterprise user id)
    #[arg(value_name = "MEMBER_ID")]
    pub member_id: String,
}

/// 在团队中移除一个成员：`DELETE /v1/directory/groups/{group_id}/members/{member_id}`
/// （scope: `pcp:write:global:team`）。
///
/// 仅解除成员与团队的关联，不删除用户本身；返回被移除的成员对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteDirectoryGroupsByGroupIdMembersByMemberId
pub async fn run(ctx: &Ctx, args: &RemoveArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/directory/groups/{}/members/{}",
        args.group_id, args.member_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
