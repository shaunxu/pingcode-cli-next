//! 工单渠道（product-channel）资源：`pc ship product-channel <operation>`。
//!
//! 对应 `/v1/ship/products/{product_id}/channels` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ProductChannelCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc ship product-channel` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ProductChannelCommand {
    /// List ticket channels of a product (read-only) (GET /v1/ship/products/{product_id}/channels)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdChannels
    List(ListArgs),
    /// Get a ticket channel by id (read-only) (GET /v1/ship/products/{product_id}/channels/{channel_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdChannelsByChannelId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: ProductChannelCommand) -> anyhow::Result<()> {
    match command {
        ProductChannelCommand::List(args) => list::run(ctx, &args).await,
        ProductChannelCommand::Get(args) => get::run(ctx, &args).await,
    }
}
