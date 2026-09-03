//! 需求排期（product-plan）资源：`pc ship product-plan <operation>`。
//!
//! 对应 `/v1/ship/products/{product_id}/plans` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ProductPlanCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc ship product-plan` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ProductPlanCommand {
    /// List requirement plans of a product (read-only) (GET /v1/ship/products/{product_id}/plans)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdPlans
    List(ListArgs),
    /// Get a requirement plan by id (read-only) (GET /v1/ship/products/{product_id}/plans/{plan_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdPlansByPlanId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: ProductPlanCommand) -> anyhow::Result<()> {
    match command {
        ProductPlanCommand::List(args) => list::run(ctx, &args).await,
        ProductPlanCommand::Get(args) => get::run(ctx, &args).await,
    }
}
