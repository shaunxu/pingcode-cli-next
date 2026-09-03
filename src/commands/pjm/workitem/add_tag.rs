use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem add-tag` 的参数。
#[derive(Debug, Args)]
pub struct AddTagArgs {
    /// Work item id
    #[arg(value_name = "WORKITEM_ID")]
    pub workitem_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向工作项中添加一个标签：`POST /v1/pjm/workitems/{workitem_id}/tags`
/// （scope: `pcp:write:pjm:workitem`）。
///
/// 请求体必填 `tag_id`（标签的 id，标签字典见 `pc pjm workitem-tag`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemsByWorkitemIdTags
pub async fn run(ctx: &Ctx, args: &AddTagArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/pjm/workitems/{}/tags", args.workitem_id);
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
