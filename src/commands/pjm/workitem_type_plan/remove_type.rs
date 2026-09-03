use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-type-plan remove-type` 的参数。
#[derive(Debug, Args)]
pub struct RemoveTypeArgs {
    /// Work item type plan id
    #[arg(value_name = "TYPE_PLAN_ID")]
    pub workitem_type_plan_id: String,

    /// Work item type id
    #[arg(value_name = "WORKITEM_TYPE_ID")]
    pub workitem_type_id: String,
}

/// 在工作项类型方案中移除一个类型：
/// `DELETE /v1/pjm/workitem_type_plans/{workitem_type_plan_id}/workitem_types/{workitem_type_id}`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemTypePlansByWorkitemTypePlanIdWorkitemTypesByWorkitemTypeId
pub async fn run(ctx: &Ctx, args: &RemoveTypeArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/workitem_type_plans/{}/workitem_types/{}",
        args.workitem_type_plan_id, args.workitem_type_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
