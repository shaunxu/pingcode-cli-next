use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc wiki space-member remove` 的参数。
#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Space id
    #[arg(value_name = "SPACE_ID")]
    pub space_id: String,

    /// Member id (enterprise user id or team id)
    #[arg(value_name = "MEMBER_ID")]
    pub member_id: String,
}

/// 在空间中移除一个成员：`DELETE /v1/wiki/spaces/{space_id}/members/{member_id}`
/// （scope: `pcp:write:wiki:space`）。
///
/// 仅解除成员与空间的关联，不删除用户或团队本身；返回被移除的成员对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteWikiSpacesBySpaceIdMembersByMemberId
pub async fn run(ctx: &Ctx, args: &RemoveArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/wiki/spaces/{}/members/{}",
        args.space_id, args.member_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
