//! 需求流转记录（idea transition history）资源（只读）：
//! `pc ship idea-transition <operation>`。
//!
//! 对应 `/v1/ship/ideas/{idea_id}/transition_histories` 及其直接子路径
//! 的 REST 接口，记录需求状态流转历史。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`IdeaTransitionCommand`] 枚举加一个变体，并在 [`run`] 的
//!    match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc ship idea-transition` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum IdeaTransitionCommand {
    /// List transition histories of an idea (GET /v1/ship/ideas/{idea_id}/transition_histories)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeasByIdeaIdTransitionHistories
    List(ListArgs),

    /// Get a transition history by id (GET /v1/ship/ideas/{idea_id}/transition_histories/{transition_history_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeasByIdeaIdTransitionHistoriesByTransitionHistoryId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: IdeaTransitionCommand) -> anyhow::Result<()> {
    match command {
        IdeaTransitionCommand::List(args) => list::run(ctx, &args).await,
        IdeaTransitionCommand::Get(args) => get::run(ctx, &args).await,
    }
}
