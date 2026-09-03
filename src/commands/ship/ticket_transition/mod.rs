//! 工单流转记录（ticket transition history）资源（只读）：
//! `pc ship ticket-transition <operation>`。
//!
//! 对应 `/v1/ship/tickets/{ticket_id}/transition_histories` 及其直接子路径
//! 的 REST 接口，记录工单状态流转历史。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TicketTransitionCommand`] 枚举加一个变体，并在 [`run`] 的
//!    match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc ship ticket-transition` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TicketTransitionCommand {
    /// List transition histories of a ticket (GET /v1/ship/tickets/{ticket_id}/transition_histories)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketsByTicketIdTransitionHistories
    List(ListArgs),

    /// Get a transition history by id (GET /v1/ship/tickets/{ticket_id}/transition_histories/{transition_history_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketsByTicketIdTransitionHistoriesByTransitionHistoryId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: TicketTransitionCommand) -> anyhow::Result<()> {
    match command {
        TicketTransitionCommand::List(args) => list::run(ctx, &args).await,
        TicketTransitionCommand::Get(args) => get::run(ctx, &args).await,
    }
}
