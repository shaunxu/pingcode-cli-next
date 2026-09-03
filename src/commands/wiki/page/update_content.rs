use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc wiki page update-content` 的参数。
#[derive(Debug, Args)]
pub struct UpdateContentArgs {
    /// Page id
    #[arg(value_name = "PAGE_ID")]
    pub page_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 更新一个文档正文：`PUT /v1/wiki/pages/{page_id}/content`
/// （scope: `pcp:write:wiki:page`）。
///
/// 请求体必填 `content`（正文内容）与 `format_type`（正文格式，
/// 取值 text/markdown/html）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/putWikiPagesByPageIdContent
pub async fn run(ctx: &Ctx, args: &UpdateContentArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/wiki/pages/{}/content", args.page_id);
    let response: Value = ctx.client.put(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
