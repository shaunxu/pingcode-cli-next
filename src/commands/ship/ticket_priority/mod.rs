//! 工单优先级（ticket-priority）资源：`pc ship ticket-priority <operation>`。
//!
//! 对应 `/v1/ship/ticket_priorities` 的 REST 接口（scope 为 `pcp:(read|write):ship:configuration`）。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TicketPriorityCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;
pub mod list_for_product;

use get::GetArgs;
use list::ListArgs;
use list_for_product::ListForProductArgs;

/// `pc ship ticket-priority` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TicketPriorityCommand {
    /// List all 工单优先级 (GET /v1/ship/ticket_priorities)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketPriorities
    List(ListArgs),
    /// List 工单优先级 in a product (GET /v1/ship/ticket/priorities?product_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketPrioritiesByProductId
    ListForProduct(ListForProductArgs),
    /// Get a 工单优先级 by id (GET /v1/ship/ticket_priorities/{priority_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketPrioritiesByPriorityId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: TicketPriorityCommand) -> anyhow::Result<()> {
    match command {
        TicketPriorityCommand::List(args) => list::run(ctx, &args).await,
        TicketPriorityCommand::ListForProduct(args) => list_for_product::run(ctx, &args).await,
        TicketPriorityCommand::Get(args) => get::run(ctx, &args).await,
    }
}
