use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship idea-property-plan list-properties` 的参数。
#[derive(Debug, Args)]
pub struct ListPropertiesArgs {
    /// Idea property plan id
    #[arg(value_name = "PROPERTY_PLAN_ID")]
    pub property_plan_id: String,
}

/// 获取需求属性方案中的属性列表：`GET /v1/ship/idea_property_plans/{property_plan_id}/idea_properties`
/// （分页，scope: `pcp:read:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaPropertyPlansByPropertyPlanIdIdeaProperties
pub async fn run(ctx: &Ctx, args: &ListPropertiesArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/idea_property_plans/{property_plan_id}/idea_properties",
        property_plan_id = args.property_plan_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
