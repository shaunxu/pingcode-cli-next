use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-type-plan update-type` 的参数。
#[derive(Debug, Args)]
pub struct UpdateTypeArgs {
    /// Work item type plan id
    #[arg(value_name = "TYPE_PLAN_ID")]
    pub workitem_type_plan_id: String,

    /// Work item type id
    #[arg(value_name = "WORKITEM_TYPE_ID")]
    pub workitem_type_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新工作项类型方案中的一个类型：
/// `PATCH /v1/pjm/workitem_type_plans/{workitem_type_plan_id}/workitem_types/{workitem_type_id}`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmWorkitemTypePlansByWorkitemTypePlanIdWorkitemTypesByWorkitemTypeId
pub async fn run(ctx: &Ctx, args: &UpdateTypeArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/pjm/workitem_type_plans/{}/workitem_types/{}",
        args.workitem_type_plan_id, args.workitem_type_id
    );
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
