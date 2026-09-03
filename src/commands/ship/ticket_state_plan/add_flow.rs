use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-state-plan add-flow` 的参数。
#[derive(Debug, Args)]
pub struct AddFlowArgs {
    /// Ticket state plan id
    #[arg(value_name = "STATE_PLAN_ID")]
    pub state_plan_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向工单状态方案中添加一个状态流转：`POST /v1/ship/ticket_state_plans/{state_plan_id}/ticket_state_flows`
/// （scope: `pcp:write:ship:configuration`）。
///
/// 请求体必填 `from_state_id`（起始状态 id）、`to_state_id`（目标状态 id），完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postShipTicketStatePlansByStatePlanIdTicketStateFlows
pub async fn run(ctx: &Ctx, args: &AddFlowArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;
    let path = format!(
        "/v1/ship/ticket_state_plans/{state_plan_id}/ticket_state_flows",
        state_plan_id = args.state_plan_id
    );
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
