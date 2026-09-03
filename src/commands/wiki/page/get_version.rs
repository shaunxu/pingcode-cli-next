use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc wiki page get-version` 的参数。
#[derive(Debug, Args)]
pub struct GetVersionArgs {
    /// Page id
    #[arg(value_name = "PAGE_ID")]
    pub page_id: String,

    /// Page version id
    #[arg(value_name = "VERSION_ID")]
    pub version_id: String,
}

/// 获取一个页面版本：`GET /v1/wiki/pages/{page_id}/versions/{version_id}`
/// （scope: `pcp:read:wiki:page`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getWikiPagesByPageIdVersionsByVersionId
pub async fn run(ctx: &Ctx, args: &GetVersionArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/wiki/pages/{}/versions/{}",
        args.page_id, args.version_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
