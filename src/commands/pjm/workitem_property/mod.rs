//! 工作项属性（workitem property）资源（只读）：
//! `pc pjm workitem-property <operation>`。
//!
//! 提供工作项自定义属性字典的查询：企业维度
//! `/v1/pjm/workitem_properties`、项目+类型维度
//! `/v1/pjm/workitem/properties` 与单条 `/v1/pjm/workitem_properties/{id}`。
//! 属性的创建/修改/删除与属性方案配置属于「工作项配置」，不在本资源范围。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemPropertyCommand`] 枚举加一个变体，并在 [`run`] 的
//!    match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;
pub mod list_for_project;

use get::GetArgs;
use list::ListArgs;
use list_for_project::ListForProjectArgs;

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
}

pub async fn run(ctx: &Ctx, command: WorkitemPropertyCommand) -> anyhow::Result<()> {
    match command {
        WorkitemPropertyCommand::List(args) => list::run(ctx, &args).await,
        WorkitemPropertyCommand::ListForProject(args) => list_for_project::run(ctx, &args).await,
        WorkitemPropertyCommand::Get(args) => get::run(ctx, &args).await,
    }
}
