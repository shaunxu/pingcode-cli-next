use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-property-plan remove-property` 的参数。
#[derive(Debug, Args)]
pub struct RemovePropertyArgs {
    /// Work item property plan id
    #[arg(value_name = "PROPERTY_PLAN_ID")]
    pub property_plan_id: String,

    /// Work item property id
    #[arg(value_name = "PROPERTY_ID")]
    pub property_id: String,
}

/// 在工作项属性方案中移除一个属性：
/// `DELETE /v1/pjm/workitem_property_plans/{property_plan_id}/workitem_properties/{property_id}`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemPropertyPlansByPropertyPlanIdWorkitemPropertiesByPropertyId
pub async fn run(ctx: &Ctx, args: &RemovePropertyArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/workitem_property_plans/{}/workitem_properties/{}",
        args.property_plan_id, args.property_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
