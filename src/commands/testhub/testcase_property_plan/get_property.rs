use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase-property-plan get-property` 的参数。
#[derive(Debug, Args)]
pub struct GetPropertyArgs {
    /// Property plan id
    #[arg(value_name = "PROPERTY_PLAN_ID")]
    pub property_plan_id: String,
    /// Property id
    #[arg(value_name = "PROPERTY_ID")]
    pub property_id: String,
}

/// 获取属性方案中的一个属性关联：`GET /v1/testhub/testcase_property_plans/{property_plan_id}/testcase_properties/{property_id}`（scope: `pcp:read:testhub:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasePropertyPlansByPropertyPlanIdTestcasePropertiesByPropertyId
pub async fn run(ctx: &Ctx, args: &GetPropertyArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/testhub/testcase_property_plans/{}/testcase_properties/{}",
        args.property_plan_id, args.property_id
    );
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
