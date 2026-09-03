use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase-property-plan add-property` 的参数。
#[derive(Debug, Args)]
pub struct AddPropertyArgs {
    /// Property plan id
    #[arg(value_name = "PROPERTY_PLAN_ID")]
    pub property_plan_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向属性方案添加属性：`POST /v1/testhub/testcase_property_plans/{property_plan_id}/testcase_properties`（scope: `pcp:write:testhub:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubTestcasePropertyPlansByPropertyPlanIdTestcaseProperties
pub async fn run(ctx: &Ctx, args: &AddPropertyArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/testhub/testcase_property_plans/{}/testcase_properties",
        args.property_plan_id
    );
    let response: Value = ctx.client.post(path.as_str(), &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
