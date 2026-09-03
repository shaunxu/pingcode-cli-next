use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc wiki page update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Page id
    #[arg(value_name = "PAGE_ID")]
    pub page_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个页面：`PATCH /v1/wiki/pages/{page_id}`
/// （scope: `pcp:write:wiki:page`）。
///
/// 请求体可选 `name`（页面名称）、`parent_id`（父页面 id）、
/// `lock`（是否锁定页面，0/1）。更新正文请使用 `update-content`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchWikiPagesByPageId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/wiki/pages/{}", args.page_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
