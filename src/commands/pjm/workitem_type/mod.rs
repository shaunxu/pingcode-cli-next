//! 工作项类型（workitem type）资源：`pc pjm workitem-type <operation>`。
//!
//! 覆盖「工作项配置」中的工作项类型字典：企业维度
//! `/v1/pjm/workitem_types`（list/get/create/update/delete）与项目维度
//! `/v1/pjm/workitem/types`（list-for-project，只读）。类型在方案中的
//! 挂载/移除见 `pc pjm workitem-type-plan`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemTypeCommand`] 枚举加一个变体，并在 [`run`] 的 match
//!    中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod list_for_project;
pub mod update;

use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use list::ListArgs;
use list_for_project::ListForProjectArgs;
use update::UpdateArgs;

/// `pc pjm workitem-type` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkitemTypeCommand {
    /// List all work item types (GET /v1/pjm/workitem_types)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTypes
    List(ListArgs),

    /// List work item types in a project (GET /v1/pjm/workitem/types?project_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTypesByProjectId
    ListForProject(ListForProjectArgs),

    /// Get a work item type by id (GET /v1/pjm/workitem_types/{workitem_type_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTypesByWorkitemTypeId
    Get(GetArgs),

    /// Create a work item type (POST /v1/pjm/workitem_types)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemTypes
    Create(CreateArgs),

    /// Partially update a work item type (PATCH /v1/pjm/workitem_types/{workitem_type_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmWorkitemTypesByWorkitemTypeId
    Update(UpdateArgs),

    /// Delete a work item type (DELETE /v1/pjm/workitem_types/{workitem_type_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemTypesByWorkitemTypeId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: WorkitemTypeCommand) -> anyhow::Result<()> {
    match command {
        WorkitemTypeCommand::List(args) => list::run(ctx, &args).await,
        WorkitemTypeCommand::ListForProject(args) => list_for_project::run(ctx, &args).await,
        WorkitemTypeCommand::Get(args) => get::run(ctx, &args).await,
        WorkitemTypeCommand::Create(args) => create::run(ctx, &args).await,
        WorkitemTypeCommand::Update(args) => update::run(ctx, &args).await,
        WorkitemTypeCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
