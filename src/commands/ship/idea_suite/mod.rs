//! 需求模块（idea-suite）资源（只读，按产品）：`pc ship idea-suite <operation>`。
//!
//! 对应 `/v1/ship/idea/suites` 的 REST 接口。产品模块/排期的全量结构与写操作
//! 见 `pc ship product-suite` / `pc ship product-plan`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`IdeaSuiteCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod list_for_product;

use list_for_product::ListForProductArgs;

/// `pc ship idea-suite` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum IdeaSuiteCommand {
    /// List idea suites (requirement modules) in a product (GET /v1/ship/idea/suites?product_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaSuitesByProductId
    ListForProduct(ListForProductArgs),
}

pub async fn run(ctx: &Ctx, command: IdeaSuiteCommand) -> anyhow::Result<()> {
    match command {
        IdeaSuiteCommand::ListForProduct(args) => list_for_product::run(ctx, &args).await,
    }
}
