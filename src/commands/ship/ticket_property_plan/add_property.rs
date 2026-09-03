use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-property-plan add-property` 的参数。
#[derive(Debug, Args)]
pub struct AddPropertyArgs {
    /// Ticket property plan id
    #[arg(value_name = "PROPERTY_PLAN_ID")]
    pub property_plan_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向工单属性方案中添加一个属性：`POST /v1/ship/ticket_property_plans/{property_plan_id}/ticket_properties`
/// （scope: `pcp:write:ship:configuration`）。
///
/// 请求体字段见官方文档（通常为属性引用及方案内配置）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postShipTicketPropertyPlansByPropertyPlanIdTicketProperties
pub async fn run(ctx: &Ctx, args: &AddPropertyArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;
    let path = format!(
        "/v1/ship/ticket_property_plans/{property_plan_id}/ticket_properties",
        property_plan_id = args.property_plan_id
    );
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
