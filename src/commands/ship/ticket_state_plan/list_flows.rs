use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-state-plan list-flows` 的参数。
#[derive(Debug, Args)]
pub struct ListFlowsArgs {
    /// Ticket state plan id
    #[arg(value_name = "STATE_PLAN_ID")]
    pub state_plan_id: String,
}

/// 获取工单状态方案中的状态流转列表：`GET /v1/ship/ticket_state_plans/{state_plan_id}/ticket_state_flows`
/// （分页，scope: `pcp:read:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketStatePlansByStatePlanIdTicketStateFlows
pub async fn run(ctx: &Ctx, args: &ListFlowsArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/ticket_state_plans/{state_plan_id}/ticket_state_flows",
        state_plan_id = args.state_plan_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
