//! 工单类型（ticket-type）资源：`pc ship ticket-type <operation>`。
//!
//! 对应 `/v1/ship/ticket_types` 的 REST 接口（scope 为 `pcp:(read|write):ship:configuration`）。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TicketTypeCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;
pub mod list_for_product;

use get::GetArgs;
use list::ListArgs;
use list_for_product::ListForProductArgs;

/// `pc ship ticket-type` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TicketTypeCommand {
    /// List all 工单类型 (GET /v1/ship/ticket_types)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketTypes
    List(ListArgs),
    /// List 工单类型 in a product (GET /v1/ship/ticket/types?product_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketTypesByProductId
    ListForProduct(ListForProductArgs),
    /// Get a 工单类型 by id (GET /v1/ship/ticket_types/{ticket_type_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketTypesByTicketTypeId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: TicketTypeCommand) -> anyhow::Result<()> {
    match command {
        TicketTypeCommand::List(args) => list::run(ctx, &args).await,
        TicketTypeCommand::ListForProduct(args) => list_for_product::run(ctx, &args).await,
        TicketTypeCommand::Get(args) => get::run(ctx, &args).await,
    }
}
