//! 工单类型（product-ticket-type）资源：`pc ship product-ticket-type <operation>`。
//!
//! 对应 `/v1/ship/products/{product_id}/ticket_types` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ProductTicketTypeCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc ship product-ticket-type` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ProductTicketTypeCommand {
    /// List ticket types of a product (read-only) (GET /v1/ship/products/{product_id}/ticket_types)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdTicketTypes
    List(ListArgs),
    /// Get a ticket type by id (read-only) (GET /v1/ship/products/{product_id}/ticket_types/{ticket_type_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdTicketTypesByTicketTypeId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: ProductTicketTypeCommand) -> anyhow::Result<()> {
    match command {
        ProductTicketTypeCommand::List(args) => list::run(ctx, &args).await,
        ProductTicketTypeCommand::Get(args) => get::run(ctx, &args).await,
    }
}
