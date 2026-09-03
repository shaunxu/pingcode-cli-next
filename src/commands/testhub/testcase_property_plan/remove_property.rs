use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase-property-plan remove-property` 的参数。
#[derive(Debug, Args)]
pub struct RemovePropertyArgs {
    /// Property plan id
    #[arg(value_name = "PROPERTY_PLAN_ID")]
    pub property_plan_id: String,
    /// Property id
    #[arg(value_name = "PROPERTY_ID")]
    pub property_id: String,
}

/// 从属性方案移除属性：`DELETE /v1/testhub/testcase_property_plans/{property_plan_id}/testcase_properties/{property_id}`（scope: `pcp:write:testhub:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteTesthubTestcasePropertyPlansByPropertyPlanIdTestcasePropertiesByPropertyId
pub async fn run(ctx: &Ctx, args: &RemovePropertyArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/testhub/testcase_property_plans/{}/testcase_properties/{}",
        args.property_plan_id, args.property_id
    );
    let response: Value = ctx.client.delete(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
