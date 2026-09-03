use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-property-plan list-properties` 的参数。
#[derive(Debug, Args)]
pub struct ListPropertiesArgs {
    /// Work item property plan id
    #[arg(value_name = "PROPERTY_PLAN_ID")]
    pub property_plan_id: String,
}

/// 获取工作项属性方案中的属性列表：
/// `GET /v1/pjm/workitem_property_plans/{property_plan_id}/workitem_properties`
/// （分页，scope: `pcp:read:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPropertyPlansByPropertyPlanIdWorkitemProperties
pub async fn run(ctx: &Ctx, args: &ListPropertiesArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/workitem_property_plans/{}/workitem_properties",
        args.property_plan_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
