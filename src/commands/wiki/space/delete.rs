use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc wiki space delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Space id
    #[arg(value_name = "SPACE_ID")]
    pub space_id: String,
}

/// 删除一个空间：`DELETE /v1/wiki/spaces/{space_id}`
/// （scope: `pcp:write:wiki:space`）。
///
/// 按 id 删除一个空间，返回被删除的空间对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteWikiSpacesBySpaceId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!("/v1/wiki/spaces/{}", args.space_id);
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
