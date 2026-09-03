use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-state get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Ticket state id
    #[arg(value_name = "TICKET_STATE_ID")]
    pub ticket_state_id: String,
}

/// 获取一个工单状态：`GET /v1/ship/ticket_states/{ticket_state_id}`（scope: `pcp:read:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketStatesByTicketStateId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/ticket_states/{ticket_state_id}",
        ticket_state_id = args.ticket_state_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
