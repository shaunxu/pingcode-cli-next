use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-tag create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个工作项标签：`POST /v1/pjm/workitem_tags`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 请求体必填 `name`（标签名称，在一个企业中唯一）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemTags
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/pjm/workitem_tags", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
