use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc wiki page create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个页面：`POST /v1/wiki/pages`（scope: `pcp:write:wiki:page`）。
///
/// 请求体必填 `space_id`（空间 id）、`name`（页面名称）；可选 `parent_id`
/// （父页面 id）、`content` 与 `format_type`（页面正文及其格式，二者必须
/// 同时传递；`format_type` 取值 text/markdown/html）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postWikiPages
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/wiki/pages", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
