use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc wiki space-member get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Space id
    #[arg(value_name = "SPACE_ID")]
    pub space_id: String,

    /// Member id (enterprise user id or team id)
    #[arg(value_name = "MEMBER_ID")]
    pub member_id: String,
}

/// 获取空间中的一个成员：`GET /v1/wiki/spaces/{space_id}/members/{member_id}`
/// （scope: `pcp:read:wiki:space`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getWikiSpacesBySpaceIdMembersByMemberId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/wiki/spaces/{}/members/{}",
        args.space_id, args.member_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
