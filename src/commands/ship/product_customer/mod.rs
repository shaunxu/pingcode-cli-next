//! 客户（product-customer）资源：`pc ship product-customer <operation>`。
//!
//! 对应 `/v1/ship/products/{product_id}/customers` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ProductCustomerCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

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

/// `pc ship product-customer` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ProductCustomerCommand {
    /// List customers of a product (GET /v1/ship/products/{product_id}/customers)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdCustomers
    List(ListArgs),
    /// Get a customer by id (GET /v1/ship/products/{product_id}/customers/{customer_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdCustomersByCustomerId
    Get(GetArgs),
    /// Create a customer in a product (POST /v1/ship/products/{product_id}/customers)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipProductsByProductIdCustomers
    Create(CreateArgs),
    /// Partially update a customer (PATCH /v1/ship/products/{product_id}/customers/{customer_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchShipProductsByProductIdCustomersByCustomerId
    Update(UpdateArgs),
}

pub async fn run(ctx: &Ctx, command: ProductCustomerCommand) -> anyhow::Result<()> {
    match command {
        ProductCustomerCommand::List(args) => list::run(ctx, &args).await,
        ProductCustomerCommand::Get(args) => get::run(ctx, &args).await,
        ProductCustomerCommand::Create(args) => create::run(ctx, &args).await,
        ProductCustomerCommand::Update(args) => update::run(ctx, &args).await,
    }
}
