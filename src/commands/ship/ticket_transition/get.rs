use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-transition get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Ticket id
    #[arg(value_name = "TICKET_ID")]
    pub ticket_id: String,

    /// Transition history id
    #[arg(value_name = "TRANSITION_HISTORY_ID")]
    pub transition_history_id: String,
}

/// 获取一条工单流转记录：
/// `GET /v1/ship/tickets/{ticket_id}/transition_histories/{transition_history_id}`
/// （scope: `pcp:read:ship:ticket`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketsByTicketIdTransitionHistoriesByTransitionHistoryId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/tickets/{}/transition_histories/{}",
        args.ticket_id, args.transition_history_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
