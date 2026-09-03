use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-state-plan get-flow` 的参数。
#[derive(Debug, Args)]
pub struct GetFlowArgs {
    /// Ticket state plan id
    #[arg(value_name = "STATE_PLAN_ID")]
    pub state_plan_id: String,
    /// Ticket state flow id
    #[arg(value_name = "STATE_FLOW_ID")]
    pub state_flow_id: String,
}

/// 获取工单状态方案中的一个状态流转：`GET /v1/ship/ticket_state_plans/{state_plan_id}/ticket_state_flows/{state_flow_id}`
/// （scope: `pcp:read:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketStatePlansByStatePlanIdTicketStateFlowsByStateFlowId
pub async fn run(ctx: &Ctx, args: &GetFlowArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/ticket_state_plans/{state_plan_id}/ticket_state_flows/{state_flow_id}",
        state_plan_id = args.state_plan_id,
        state_flow_id = args.state_flow_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
