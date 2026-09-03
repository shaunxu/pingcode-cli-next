//! 需求状态（idea-state）资源（只读）：`pc ship idea-state <operation>`。
//!
//! 对应 `/v1/ship/idea_states` 与 `/v1/ship/idea/states` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`IdeaStateCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;
pub mod list_for_product;

use get::GetArgs;
use list::ListArgs;
use list_for_product::ListForProductArgs;

/// `pc ship idea-state` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum IdeaStateCommand {
    /// List all 需求状态 (GET /v1/ship/idea_states)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaStates
    List(ListArgs),

    /// List 需求状态 in a product (GET /v1/ship/idea/states?product_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaStatesByProductId
    ListForProduct(ListForProductArgs),

    /// Get a 需求状态 by id (GET /v1/ship/idea_states/{idea_state_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaStatesByIdeaStateId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: IdeaStateCommand) -> anyhow::Result<()> {
    match command {
        IdeaStateCommand::List(args) => list::run(ctx, &args).await,
        IdeaStateCommand::ListForProduct(args) => list_for_product::run(ctx, &args).await,
        IdeaStateCommand::Get(args) => get::run(ctx, &args).await,
    }
}
