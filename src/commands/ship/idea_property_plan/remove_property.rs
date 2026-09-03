use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship idea-property-plan remove-property` 的参数。
#[derive(Debug, Args)]
pub struct RemovePropertyArgs {
    /// Idea property plan id
    #[arg(value_name = "PROPERTY_PLAN_ID")]
    pub property_plan_id: String,
    /// Idea property id
    #[arg(value_name = "PROPERTY_ID")]
    pub property_id: String,
}

/// 在需求属性方案中移除一个属性：`DELETE /v1/ship/idea_property_plans/{property_plan_id}/idea_properties/{property_id}`
/// （scope: `pcp:write:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteShipIdeaPropertyPlansByPropertyPlanIdIdeaPropertiesByPropertyId
pub async fn run(ctx: &Ctx, args: &RemovePropertyArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/idea_property_plans/{property_plan_id}/idea_properties/{property_id}",
        property_plan_id = args.property_plan_id,
        property_id = args.property_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
