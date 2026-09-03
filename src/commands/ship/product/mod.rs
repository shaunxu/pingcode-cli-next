//! 产品（product）资源：`pc ship product <operation>`。
//!
//! 对应 `/v1/ship/products` 及其直接子路径的 REST 接口（不含产品下的
//! 成员、模块、标签、外部用户、客户、渠道、排期、工单类型等子资源，
//! 这些分别有独立的资源目录）。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ProductCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod get;
pub mod list;
pub mod update;

use create::CreateArgs;
use get::GetArgs;
use list::ListArgs;
use update::UpdateArgs;

/// `pc ship product` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ProductCommand {
    /// List products (GET /v1/ship/products)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProducts
    List(ListArgs),

    /// Get a product by id (GET /v1/ship/products/{product_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductId
    Get(GetArgs),

    /// Create a product (POST /v1/ship/products)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipProducts
    Create(CreateArgs),

    /// Partially update a product (PATCH /v1/ship/products/{product_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchShipProductsByProductId
    Update(UpdateArgs),
}

pub async fn run(ctx: &Ctx, command: ProductCommand) -> anyhow::Result<()> {
    match command {
        ProductCommand::List(args) => list::run(ctx, &args).await,
        ProductCommand::Get(args) => get::run(ctx, &args).await,
        ProductCommand::Create(args) => create::run(ctx, &args).await,
        ProductCommand::Update(args) => update::run(ctx, &args).await,
    }
}
