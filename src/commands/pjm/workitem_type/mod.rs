//! 工作项类型（workitem type）资源（只读）：
//! `pc pjm workitem-type <operation>`。
//!
//! 提供工作项类型字典的查询：企业维度 `/v1/pjm/workitem_types` 与
//! 项目维度 `/v1/pjm/workitem/types`。类型的创建/修改/删除属于
//! 「工作项配置」，不在本资源范围。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemTypeCommand`] 枚举加一个变体，并在 [`run`] 的 match
//!    中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;
pub mod list_for_project;

use get::GetArgs;
use list::ListArgs;
use list_for_project::ListForProjectArgs;

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
}

pub async fn run(ctx: &Ctx, command: WorkitemTypeCommand) -> anyhow::Result<()> {
    match command {
        WorkitemTypeCommand::List(args) => list::run(ctx, &args).await,
        WorkitemTypeCommand::ListForProject(args) => list_for_project::run(ctx, &args).await,
        WorkitemTypeCommand::Get(args) => get::run(ctx, &args).await,
    }
}
