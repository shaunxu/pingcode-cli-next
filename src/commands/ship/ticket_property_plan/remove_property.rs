use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-property-plan remove-property` 的参数。
#[derive(Debug, Args)]
pub struct RemovePropertyArgs {
    /// Ticket property plan id
    #[arg(value_name = "PROPERTY_PLAN_ID")]
    pub property_plan_id: String,
    /// Ticket property id
    #[arg(value_name = "PROPERTY_ID")]
    pub property_id: String,
}

/// 在工单属性方案中移除一个属性：`DELETE /v1/ship/ticket_property_plans/{property_plan_id}/ticket_properties/{property_id}`
/// （scope: `pcp:write:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteShipTicketPropertyPlansByPropertyPlanIdTicketPropertiesByPropertyId
pub async fn run(ctx: &Ctx, args: &RemovePropertyArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/ticket_property_plans/{property_plan_id}/ticket_properties/{property_id}",
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
