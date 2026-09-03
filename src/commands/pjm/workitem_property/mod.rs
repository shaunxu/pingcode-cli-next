//! 工作项属性（workitem property）资源：
//! `pc pjm workitem-property <operation>`。
//!
//! 覆盖「工作项配置」中的工作项自定义属性字典：企业维度
//! `/v1/pjm/workitem_properties`（list/get/create/update）与项目+类型
//! 维度 `/v1/pjm/workitem/properties`（list-for-project，只读）。属性在
//! 方案中的挂载/移除见 `pc pjm workitem-property-plan`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemPropertyCommand`] 枚举加一个变体，并在 [`run`] 的
//!    match 中加一行分发。

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

/// `pc pjm workitem-property` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkitemPropertyCommand {
    /// List all work item properties (GET /v1/pjm/workitem_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemProperties
    List(ListArgs),

    /// List work item properties for a project and type (GET /v1/pjm/workitem/properties?project_id=...&workitem_type_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPropertiesByProjectIdAndWorkitemTypeId
    ListForProject(ListForProjectArgs),

    /// Get a work item property by id (GET /v1/pjm/workitem_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPropertiesByPropertyId
    Get(GetArgs),

    /// Create a work item property (POST /v1/pjm/workitem_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemProperties
    Create(CreateArgs),

    /// Partially update a work item property (PATCH /v1/pjm/workitem_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmWorkitemPropertiesByPropertyId
    Update(UpdateArgs),
}

pub async fn run(ctx: &Ctx, command: WorkitemPropertyCommand) -> anyhow::Result<()> {
    match command {
        WorkitemPropertyCommand::List(args) => list::run(ctx, &args).await,
        WorkitemPropertyCommand::ListForProject(args) => list_for_project::run(ctx, &args).await,
        WorkitemPropertyCommand::Get(args) => get::run(ctx, &args).await,
        WorkitemPropertyCommand::Create(args) => create::run(ctx, &args).await,
        WorkitemPropertyCommand::Update(args) => update::run(ctx, &args).await,
    }
}
