//! 标签（product-tag）资源：`pc ship product-tag <operation>`。
//!
//! 对应 `/v1/ship/products/{product_id}/tags` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ProductTagCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;

use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use list::ListArgs;

/// `pc ship product-tag` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ProductTagCommand {
    /// List tags of a product (GET /v1/ship/products/{product_id}/tags)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdTags
    List(ListArgs),
    /// Get a product tag by id (GET /v1/ship/products/{product_id}/tags/{tag_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdTagsByTagId
    Get(GetArgs),
    /// Create a tag in a product (POST /v1/ship/products/{product_id}/tags)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipProductsByProductIdTags
    Create(CreateArgs),
    /// Delete a tag from a product (DELETE /v1/ship/products/{product_id}/tags/{tag_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteShipProductsByProductIdTagsByTagId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: ProductTagCommand) -> anyhow::Result<()> {
    match command {
        ProductTagCommand::List(args) => list::run(ctx, &args).await,
        ProductTagCommand::Get(args) => get::run(ctx, &args).await,
        ProductTagCommand::Create(args) => create::run(ctx, &args).await,
        ProductTagCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
