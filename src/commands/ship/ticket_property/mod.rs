//! 工单属性（ticket-property）资源：`pc ship ticket-property <operation>`。
//!
//! 对应 `/v1/ship/ticket_properties` 的 REST 接口（scope 为 `pcp:(read|write):ship:configuration`）。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TicketPropertyCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod get;
pub mod list;
pub mod list_for_product;
pub mod update;

use create::CreateArgs;
use get::GetArgs;
use list::ListArgs;
use list_for_product::ListForProductArgs;
use update::UpdateArgs;

/// `pc ship ticket-property` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TicketPropertyCommand {
    /// List all 工单属性 (GET /v1/ship/ticket_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketProperties
    List(ListArgs),
    /// List 工单属性 in a product (GET /v1/ship/ticket/properties?product_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketPropertiesByProductId
    ListForProduct(ListForProductArgs),
    /// Get a 工单属性 by id (GET /v1/ship/ticket_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketPropertiesByPropertyId
    Get(GetArgs),
    /// Create a 工单属性 (POST /v1/ship/ticket_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipTicketProperties
    Create(CreateArgs),
    /// Partially update a 工单属性 (PATCH /v1/ship/ticket_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchShipTicketPropertiesByPropertyId
    Update(UpdateArgs),
}

pub async fn run(ctx: &Ctx, command: TicketPropertyCommand) -> anyhow::Result<()> {
    match command {
        TicketPropertyCommand::List(args) => list::run(ctx, &args).await,
        TicketPropertyCommand::ListForProduct(args) => list_for_product::run(ctx, &args).await,
        TicketPropertyCommand::Get(args) => get::run(ctx, &args).await,
        TicketPropertyCommand::Create(args) => create::run(ctx, &args).await,
        TicketPropertyCommand::Update(args) => update::run(ctx, &args).await,
    }
}
