//! 工单解决方案（ticket-solution）资源：`pc ship ticket-solution <operation>`。
//!
//! 对应 `/v1/ship/ticket_solutions` 的 REST 接口（scope 为 `pcp:(read|write):ship:configuration`）。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TicketSolutionCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;
pub mod list_for_product;

use get::GetArgs;
use list::ListArgs;
use list_for_product::ListForProductArgs;

/// `pc ship ticket-solution` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TicketSolutionCommand {
    /// List all 工单解决方案 (GET /v1/ship/ticket_solutions)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketSolutions
    List(ListArgs),
    /// List 工单解决方案 in a product (GET /v1/ship/ticket/solutions?product_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketSolutionsByProductId
    ListForProduct(ListForProductArgs),
    /// Get a 工单解决方案 by id (GET /v1/ship/ticket_solutions/{ticket_solution_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketSolutionsByTicketSolutionId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: TicketSolutionCommand) -> anyhow::Result<()> {
    match command {
        TicketSolutionCommand::List(args) => list::run(ctx, &args).await,
        TicketSolutionCommand::ListForProduct(args) => list_for_product::run(ctx, &args).await,
        TicketSolutionCommand::Get(args) => get::run(ctx, &args).await,
    }
}
