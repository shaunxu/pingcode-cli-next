use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-type-plan get-type` 的参数。
#[derive(Debug, Args)]
pub struct GetTypeArgs {
    /// Work item type plan id
    #[arg(value_name = "TYPE_PLAN_ID")]
    pub workitem_type_plan_id: String,

    /// Work item type id
    #[arg(value_name = "WORKITEM_TYPE_ID")]
    pub workitem_type_id: String,
}

/// 获取工作项类型方案中的一个类型：
/// `GET /v1/pjm/workitem_type_plans/{workitem_type_plan_id}/workitem_types/{workitem_type_id}`
/// （scope: `pcp:read:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTypePlansByWorkitemTypePlanIdWorkitemTypesByWorkitemTypeId
pub async fn run(ctx: &Ctx, args: &GetTypeArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/workitem_type_plans/{}/workitem_types/{}",
        args.workitem_type_plan_id, args.workitem_type_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
