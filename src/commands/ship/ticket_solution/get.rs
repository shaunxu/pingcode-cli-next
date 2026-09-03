use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-solution get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Ticket solution id
    #[arg(value_name = "TICKET_SOLUTION_ID")]
    pub ticket_solution_id: String,
}

/// 获取一个工单解决方案：`GET /v1/ship/ticket_solutions/{ticket_solution_id}`（scope: `pcp:read:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketSolutionsByTicketSolutionId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/ticket_solutions/{ticket_solution_id}",
        ticket_solution_id = args.ticket_solution_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
