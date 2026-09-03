//! 工作项状态（workitem state）资源：`pc pjm workitem-state <operation>`。
//!
//! 覆盖「工作项配置」中的工作项状态字典：企业维度
//! `/v1/pjm/workitem_states`（list-all/get/create/update）与项目+类型
//! 维度 `/v1/pjm/workitem/states`（list-for-project，只读）。状态在方案
//! 中的挂载/移除、状态流转配置见 `pc pjm workitem-state-plan`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemStateCommand`] 枚举加一个变体，并在 [`run`] 的 match
//!    中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod get;
pub mod list;
pub mod list_for_project;
pub mod update;

use create::CreateArgs;
use get::GetArgs;
use list::ListArgs;
use list_for_project::ListForProjectArgs;
use update::UpdateArgs;

/// `pc pjm workitem-state` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkitemStateCommand {
    /// List all work item states (GET /v1/pjm/workitem_states)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemStates
    ListAll(ListArgs),

    /// List work item states for a project and type (GET /v1/pjm/workitem/states?project_id=...&workitem_type_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemStatesByProjectIdAndWorkitemTypeId
    ListForProject(ListForProjectArgs),

    /// Get a work item state by id (GET /v1/pjm/workitem_states/{state_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemStatesByStateId
    Get(GetArgs),

    /// Create a work item state (POST /v1/pjm/workitem_states)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemStates
    Create(CreateArgs),

    /// Partially update a work item state (PATCH /v1/pjm/workitem_states/{state_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmWorkitemStatesByStateId
    Update(UpdateArgs),
}

pub async fn run(ctx: &Ctx, command: WorkitemStateCommand) -> anyhow::Result<()> {
    match command {
        WorkitemStateCommand::ListAll(args) => list::run(ctx, &args).await,
        WorkitemStateCommand::ListForProject(args) => list_for_project::run(ctx, &args).await,
        WorkitemStateCommand::Get(args) => get::run(ctx, &args).await,
        WorkitemStateCommand::Create(args) => create::run(ctx, &args).await,
        WorkitemStateCommand::Update(args) => update::run(ctx, &args).await,
    }
}
