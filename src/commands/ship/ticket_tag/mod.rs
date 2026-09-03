//! 工单标签（ticket tag）资源（只读）：`pc ship ticket-tag <operation>`。
//!
//! 对应 `/v1/ship/ticket/tags` 的 REST 接口（按产品查询工单标签字典）。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TicketTagCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod list_for_product;

use list_for_product::ListForProductArgs;

/// `pc ship ticket-tag` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TicketTagCommand {
    /// List ticket tags in a product (GET /v1/ship/ticket/tags?product_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketTagsByProductId
    ListForProduct(ListForProductArgs),
}

pub async fn run(ctx: &Ctx, command: TicketTagCommand) -> anyhow::Result<()> {
    match command {
        TicketTagCommand::ListForProduct(args) => list_for_product::run(ctx, &args).await,
    }
}
