//! 工作项优先级（workitem priority）资源（只读）：
//! `pc pjm workitem-priority <operation>`。
//!
//! 提供工作项优先级字典的查询：企业维度
//! `/v1/pjm/workitem_priorities`、项目维度
//! `/v1/pjm/workitem/priorities` 与单条
//! `/v1/pjm/workitem_priorities/{id}`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemPriorityCommand`] 枚举加一个变体，并在 [`run`] 的
//!    match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;
pub mod list_for_project;

use get::GetArgs;
use list::ListArgs;
use list_for_project::ListForProjectArgs;

/// `pc pjm workitem-priority` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkitemPriorityCommand {
    /// List all work item priorities (GET /v1/pjm/workitem_priorities)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPriorities
    List(ListArgs),

    /// List work item priorities in a project (GET /v1/pjm/workitem/priorities?project_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPrioritiesByProjectId
    ListForProject(ListForProjectArgs),

    /// Get a work item priority by id (GET /v1/pjm/workitem_priorities/{priority_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPrioritiesByPriorityId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: WorkitemPriorityCommand) -> anyhow::Result<()> {
    match command {
        WorkitemPriorityCommand::List(args) => list::run(ctx, &args).await,
        WorkitemPriorityCommand::ListForProject(args) => list_for_project::run(ctx, &args).await,
        WorkitemPriorityCommand::Get(args) => get::run(ctx, &args).await,
    }
}
