use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-type-plan add-type` 的参数。
#[derive(Debug, Args)]
pub struct AddTypeArgs {
    /// Work item type plan id
    #[arg(value_name = "TYPE_PLAN_ID")]
    pub workitem_type_plan_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向工作项类型方案中添加一个类型：
/// `POST /v1/pjm/workitem_type_plans/{workitem_type_plan_id}/workitem_types`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 请求体必填 `workitem_type_id`（工作项类型的 id）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemTypePlansByWorkitemTypePlanIdWorkitemTypes
pub async fn run(ctx: &Ctx, args: &AddTypeArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/pjm/workitem_type_plans/{}/workitem_types",
        args.workitem_type_plan_id
    );
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
