//! 工作项流转记录（workitem transition history）资源（只读）：
//! `pc pjm workitem-transition <operation>`。
//!
//! 对应 `/v1/pjm/workitems/{workitem_id}/transition_histories` 及其
//! 直接子路径的 REST 接口，记录工作项状态流转历史。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemTransitionCommand`] 枚举加一个变体，并在 [`run`] 的
//!    match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc pjm workitem-transition` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkitemTransitionCommand {
    /// List transition histories of a work item (GET /v1/pjm/workitems/{workitem_id}/transition_histories)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemsByWorkitemIdTransitionHistories
    List(ListArgs),

    /// Get a transition history by id (GET /v1/pjm/workitems/{workitem_id}/transition_histories/{transition_history_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemsByWorkitemIdTransitionHistoriesByTransitionHistoryId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: WorkitemTransitionCommand) -> anyhow::Result<()> {
    match command {
        WorkitemTransitionCommand::List(args) => list::run(ctx, &args).await,
        WorkitemTransitionCommand::Get(args) => get::run(ctx, &args).await,
    }
}
