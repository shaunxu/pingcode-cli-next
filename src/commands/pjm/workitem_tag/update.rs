use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-tag update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Tag id
    #[arg(value_name = "TAG_ID")]
    pub tag_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个工作项标签：`PATCH /v1/pjm/workitem_tags/{tag_id}`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 请求体可选 `name`（标签名称，在一个企业中唯一）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmWorkitemTagsByTagId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/pjm/workitem_tags/{}", args.tag_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
