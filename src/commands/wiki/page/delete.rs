use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc wiki page delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Page id
    #[arg(value_name = "PAGE_ID")]
    pub page_id: String,
}

/// 删除一个页面：`DELETE /v1/wiki/pages/{page_id}`
/// （scope: `pcp:write:wiki:page`）。
///
/// 按 id 删除一个页面，返回被删除的页面对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteWikiPagesByPageId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!("/v1/wiki/pages/{}", args.page_id);
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
