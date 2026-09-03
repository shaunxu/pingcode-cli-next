//! 需求优先级（idea-priority）资源（只读）：`pc ship idea-priority <operation>`。
//!
//! 对应 `/v1/ship/idea_priorities` 与 `/v1/ship/idea/priorities` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`IdeaPriorityCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;
pub mod list_for_product;

use get::GetArgs;
use list::ListArgs;
use list_for_product::ListForProductArgs;

/// `pc ship idea-priority` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum IdeaPriorityCommand {
    /// List all 需求优先级 (GET /v1/ship/idea_priorities)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaPriorities
    List(ListArgs),

    /// List 需求优先级 in a product (GET /v1/ship/idea/priorities?product_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaPrioritiesByProductId
    ListForProduct(ListForProductArgs),

    /// Get a 需求优先级 by id (GET /v1/ship/idea_priorities/{priority_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaPrioritiesByPriorityId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: IdeaPriorityCommand) -> anyhow::Result<()> {
    match command {
        IdeaPriorityCommand::List(args) => list::run(ctx, &args).await,
        IdeaPriorityCommand::ListForProduct(args) => list_for_product::run(ctx, &args).await,
        IdeaPriorityCommand::Get(args) => get::run(ctx, &args).await,
    }
}
