//! 工作项状态（workitem state）资源（只读）：
//! `pc pjm workitem-state <operation>`。
//!
//! 提供工作项状态字典的查询：项目+类型维度 `/v1/pjm/workitem/states`
//! 与单条 `/v1/pjm/workitem_states/{id}`。状态的创建/修改/删除与
//! 状态方案、状态流转配置属于「工作项配置」，不在本资源范围。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemStateCommand`] 枚举加一个变体，并在 [`run`] 的 match
//!    中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list_for_project;

use get::GetArgs;
use list_for_project::ListForProjectArgs;

/// `pc pjm workitem-state` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkitemStateCommand {
    /// List work item states for a project and type (GET /v1/pjm/workitem/states?project_id=...&workitem_type_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemStatesByProjectIdAndWorkitemTypeId
    ListForProject(ListForProjectArgs),

    /// Get a work item state by id (GET /v1/pjm/workitem_states/{state_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemStatesByStateId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: WorkitemStateCommand) -> anyhow::Result<()> {
    match command {
        WorkitemStateCommand::ListForProject(args) => list_for_project::run(ctx, &args).await,
        WorkitemStateCommand::Get(args) => get::run(ctx, &args).await,
    }
}
