use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub library-member remove` 的参数。
#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
    /// Member id (enterprise user id or team id)
    #[arg(value_name = "MEMBER_ID")]
    pub member_id: String,
}

/// 从测试库移除成员：`DELETE /v1/testhub/libraries/{library_id}/members/{member_id}`（scope: `pcp:write:testhub:library`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteTesthubLibrariesByLibraryIdMembersByMemberId
pub async fn run(ctx: &Ctx, args: &RemoveArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/testhub/libraries/{}/members/{}",
        args.library_id, args.member_id
    );
    let response: Value = ctx.client.delete(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
