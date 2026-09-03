//! 需求模块（product-suite）资源：`pc ship product-suite <operation>`。
//!
//! 对应 `/v1/ship/products/{product_id}/suites` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ProductSuiteCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

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

/// `pc ship product-suite` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ProductSuiteCommand {
    /// List requirement modules of a product (GET /v1/ship/products/{product_id}/suites)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdSuites
    List(ListArgs),
    /// Get a requirement module by id (GET /v1/ship/products/{product_id}/suites/{suite_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdSuitesBySuiteId
    Get(GetArgs),
    /// Create a requirement module in a product (POST /v1/ship/products/{product_id}/suites)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipProductsByProductIdSuites
    Create(CreateArgs),
    /// Delete a requirement module from a product (deletes its child modules) (DELETE /v1/ship/products/{product_id}/suites/{suite_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteShipProductsByProductIdSuitesBySuiteId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: ProductSuiteCommand) -> anyhow::Result<()> {
    match command {
        ProductSuiteCommand::List(args) => list::run(ctx, &args).await,
        ProductSuiteCommand::Get(args) => get::run(ctx, &args).await,
        ProductSuiteCommand::Create(args) => create::run(ctx, &args).await,
        ProductSuiteCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
