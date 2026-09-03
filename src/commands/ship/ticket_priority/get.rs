use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-priority get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Ticket priority id
    #[arg(value_name = "PRIORITY_ID")]
    pub priority_id: String,
}

/// 获取一个工单优先级：`GET /v1/ship/ticket_priorities/{priority_id}`（scope: `pcp:read:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketPrioritiesByPriorityId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/ticket_priorities/{priority_id}",
        priority_id = args.priority_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
