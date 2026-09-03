use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-property-plan add-property` 的参数。
#[derive(Debug, Args)]
pub struct AddPropertyArgs {
    /// Work item property plan id
    #[arg(value_name = "PROPERTY_PLAN_ID")]
    pub property_plan_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向工作项属性方案中添加一个属性：
/// `POST /v1/pjm/workitem_property_plans/{property_plan_id}/workitem_properties`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 请求体必填 `workitem_property_id`（工作项属性的 id）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemPropertyPlansByPropertyPlanIdWorkitemProperties
pub async fn run(ctx: &Ctx, args: &AddPropertyArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/pjm/workitem_property_plans/{}/workitem_properties",
        args.property_plan_id
    );
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
