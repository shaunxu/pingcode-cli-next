use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase-property-plan list-properties` 的参数。
#[derive(Debug, Args)]
pub struct ListPropertiesArgs {
    /// Property plan id
    #[arg(value_name = "PROPERTY_PLAN_ID")]
    pub property_plan_id: String,
}

/// 获取属性方案中的属性列表：`GET /v1/testhub/testcase_property_plans/{property_plan_id}/testcase_properties`（scope: `pcp:read:testhub:configuration`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasePropertyPlansByPropertyPlanIdTestcaseProperties
pub async fn run(ctx: &Ctx, args: &ListPropertiesArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/testhub/testcase_property_plans/{}/testcase_properties",
        args.property_plan_id
    );
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
