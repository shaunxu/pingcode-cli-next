use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub library-member get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
    /// Member id (enterprise user id or team id)
    #[arg(value_name = "MEMBER_ID")]
    pub member_id: String,
}

/// 获取一个测试库成员：`GET /v1/testhub/libraries/{library_id}/members/{member_id}`（scope: `pcp:read:testhub:library`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdMembersByMemberId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/testhub/libraries/{}/members/{}",
        args.library_id, args.member_id
    );
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
