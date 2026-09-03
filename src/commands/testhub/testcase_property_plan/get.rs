use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase-property-plan get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Property plan id
    #[arg(value_name = "PROPERTY_PLAN_ID")]
    pub property_plan_id: String,
}

/// 获取一个用例属性方案：`GET /v1/testhub/testcase_property_plans/{property_plan_id}`（scope: `pcp:read:testhub:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasePropertyPlansByPropertyPlanId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/testhub/testcase_property_plans/{}",
        args.property_plan_id
    );
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
