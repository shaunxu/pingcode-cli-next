use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization team get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {}

/// 获取企业信息：`GET /v1/directory/team`（scope: `pcp:read:global:team`）。
///
/// 返回当前企业（团队）的基本信息：id、url、name、secondary_domain。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryTeam
pub async fn run(ctx: &Ctx, _args: &GetArgs) -> anyhow::Result<()> {
    let response: Value = ctx.client.get("/v1/directory/team").await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
