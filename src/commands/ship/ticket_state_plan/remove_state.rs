use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-state-plan remove-state` 的参数。
#[derive(Debug, Args)]
pub struct RemoveStateArgs {
    /// Ticket state plan id
    #[arg(value_name = "STATE_PLAN_ID")]
    pub state_plan_id: String,
    /// Ticket state id
    #[arg(value_name = "STATE_ID")]
    pub state_id: String,
}

/// 在工单状态方案中移除一个状态：`DELETE /v1/ship/ticket_state_plans/{state_plan_id}/ticket_states/{state_id}`
/// （scope: `pcp:write:ship:configuration`）。每种类型的状态至少保留一个，否则无法移除。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteShipTicketStatePlansByStatePlanIdTicketStatesByStateId
pub async fn run(ctx: &Ctx, args: &RemoveStateArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/ticket_state_plans/{state_plan_id}/ticket_states/{state_id}",
        state_plan_id = args.state_plan_id,
        state_id = args.state_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
