//! 产品成员（product-member）资源：`pc ship product-member <operation>`。
//!
//! 对应 `/v1/ship/products/{product_id}/members` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ProductMemberCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod add;
pub mod get;
pub mod list;
pub mod remove;

use add::AddArgs;
use get::GetArgs;
use list::ListArgs;
use remove::RemoveArgs;

/// `pc ship product-member` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ProductMemberCommand {
    /// List members of a product (GET /v1/ship/products/{product_id}/members)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdMembers
    List(ListArgs),
    /// Get a product member by id (GET /v1/ship/products/{product_id}/members/{member_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdMembersByMemberId
    Get(GetArgs),
    /// Add a member to a product (POST /v1/ship/products/{product_id}/members)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipProductsByProductIdMembers
    Add(AddArgs),
    /// Remove a member from a product (DELETE /v1/ship/products/{product_id}/members/{member_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteShipProductsByProductIdMembersByMemberId
    Remove(RemoveArgs),
}

pub async fn run(ctx: &Ctx, command: ProductMemberCommand) -> anyhow::Result<()> {
    match command {
        ProductMemberCommand::List(args) => list::run(ctx, &args).await,
        ProductMemberCommand::Get(args) => get::run(ctx, &args).await,
        ProductMemberCommand::Add(args) => add::run(ctx, &args).await,
        ProductMemberCommand::Remove(args) => remove::run(ctx, &args).await,
    }
}
