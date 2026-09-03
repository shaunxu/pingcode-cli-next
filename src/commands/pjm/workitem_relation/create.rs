use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-relation create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Source work item id
    #[arg(value_name = "WORKITEM_ID")]
    pub workitem_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 关联一个工作项：`POST /v1/pjm/workitems/{workitem_id}/relations`
/// （scope: `pcp:write:pjm:workitem`）。
///
/// 将当前工作项与目标工作项建立指定类型的关联。请求体必填
/// `target_workitem_id`（目标工作项 id）与 `relation_type`（关联类型，
/// 如 mention/clone/duplicate/relate/cause/block/blocked_by/dependency，
/// 或自定义关联类型的 id）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemsByWorkitemIdRelations
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/pjm/workitems/{}/relations", args.workitem_id);
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
