//! 工单渠道（ticket channel）资源（只读）：`pc ship ticket-channel <operation>`。
//!
//! 对应 `/v1/ship/ticket/channels` 的 REST 接口（按产品查询工单渠道）。
//! 产品下渠道的全量结构也可通过 `pc ship product-channel` 访问。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TicketChannelCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod list_for_product;

use list_for_product::ListForProductArgs;

/// `pc ship ticket-channel` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TicketChannelCommand {
    /// List ticket channels in a product (GET /v1/ship/ticket/channels?product_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketChannelsByProductId
    ListForProduct(ListForProductArgs),
}

pub async fn run(ctx: &Ctx, command: TicketChannelCommand) -> anyhow::Result<()> {
    match command {
        TicketChannelCommand::ListForProduct(args) => list_for_product::run(ctx, &args).await,
    }
}
