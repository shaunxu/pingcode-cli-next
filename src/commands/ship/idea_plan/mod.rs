//! 需求排期（idea-plan）资源（只读，按产品）：`pc ship idea-plan <operation>`。
//!
//! 对应 `/v1/ship/idea/plans` 的 REST 接口。产品模块/排期的全量结构与写操作
//! 见 `pc ship product-suite` / `pc ship product-plan`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`IdeaPlanCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod list_for_product;

use list_for_product::ListForProductArgs;

/// `pc ship idea-plan` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum IdeaPlanCommand {
    /// List idea plans (requirement schedules) in a product (GET /v1/ship/idea/plans?product_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaPlansByProductId
    ListForProduct(ListForProductArgs),
}

pub async fn run(ctx: &Ctx, command: IdeaPlanCommand) -> anyhow::Result<()> {
    match command {
        IdeaPlanCommand::ListForProduct(args) => list_for_product::run(ctx, &args).await,
    }
}
