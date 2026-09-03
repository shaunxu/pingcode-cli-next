//! 外部用户（product-user）资源：`pc ship product-user <operation>`。
//!
//! 对应 `/v1/ship/products/{product_id}/users` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ProductUserCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use list::ListArgs;
use update::UpdateArgs;

/// `pc ship product-user` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ProductUserCommand {
    /// List external users of a product (GET /v1/ship/products/{product_id}/users)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdUsers
    List(ListArgs),
    /// Get an external user by id (GET /v1/ship/products/{product_id}/users/{user_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdUsersByUserId
    Get(GetArgs),
    /// Create an external user in a product (POST /v1/ship/products/{product_id}/users)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipProductsByProductIdUsers
    Create(CreateArgs),
    /// Partially update an external user (PATCH /v1/ship/products/{product_id}/users/{user_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchShipProductsByProductIdUsersByUserId
    Update(UpdateArgs),
    /// Delete an external user from a product (DELETE /v1/ship/products/{product_id}/users/{user_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteShipProductsByProductIdUsersByUserId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: ProductUserCommand) -> anyhow::Result<()> {
    match command {
        ProductUserCommand::List(args) => list::run(ctx, &args).await,
        ProductUserCommand::Get(args) => get::run(ctx, &args).await,
        ProductUserCommand::Create(args) => create::run(ctx, &args).await,
        ProductUserCommand::Update(args) => update::run(ctx, &args).await,
        ProductUserCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
