//! 项目属性（project property）资源：`pc pjm project-property <operation>`。
//!
//! 同时覆盖两类 REST 接口：
//! - 全局属性定义 `/v1/pjm/project_properties`（list/get/create/update）；
//! - 项目内属性配置 `/v1/pjm/projects/{project_id}/project_properties`
//!   （list-for-project / get-in-project / add-to-project / remove-from-project）。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ProjectPropertyCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod add_to_project;
pub mod create;
pub mod get;
pub mod get_in_project;
pub mod list;
pub mod list_for_project;
pub mod remove_from_project;
pub mod update;

use add_to_project::AddToProjectArgs;
use create::CreateArgs;
use get::GetArgs;
use get_in_project::GetInProjectArgs;
use list::ListArgs;
use list_for_project::ListForProjectArgs;
use remove_from_project::RemoveFromProjectArgs;
use update::UpdateArgs;

/// `pc pjm project-property` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ProjectPropertyCommand {
    /// List global project property definitions (GET /v1/pjm/project_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectProperties
    List(ListArgs),

    /// Get a global project property definition by id
    /// (GET /v1/pjm/project_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectPropertiesByPropertyId
    Get(GetArgs),

    /// Create a global project property definition (POST /v1/pjm/project_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectProperties
    Create(CreateArgs),

    /// Partially update a global project property definition
    /// (PATCH /v1/pjm/project_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectPropertiesByPropertyId
    Update(UpdateArgs),

    /// List project properties in a project
    /// (GET /v1/pjm/projects/{project_id}/project_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdProjectProperties
    ListForProject(ListForProjectArgs),

    /// Get one project property in a project
    /// (GET /v1/pjm/projects/{project_id}/project_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdProjectPropertiesByPropertyId
    GetInProject(GetInProjectArgs),

    /// Add a project property to a project
    /// (POST /v1/pjm/projects/{project_id}/project_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdProjectProperties
    AddToProject(AddToProjectArgs),

    /// Remove a project property from a project
    /// (DELETE /v1/pjm/projects/{project_id}/project_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdProjectPropertiesByPropertyId
    RemoveFromProject(RemoveFromProjectArgs),
}

pub async fn run(ctx: &Ctx, command: ProjectPropertyCommand) -> anyhow::Result<()> {
    match command {
        ProjectPropertyCommand::List(args) => list::run(ctx, &args).await,
        ProjectPropertyCommand::Get(args) => get::run(ctx, &args).await,
        ProjectPropertyCommand::Create(args) => create::run(ctx, &args).await,
        ProjectPropertyCommand::Update(args) => update::run(ctx, &args).await,
        ProjectPropertyCommand::ListForProject(args) => list_for_project::run(ctx, &args).await,
        ProjectPropertyCommand::GetInProject(args) => get_in_project::run(ctx, &args).await,
        ProjectPropertyCommand::AddToProject(args) => add_to_project::run(ctx, &args).await,
        ProjectPropertyCommand::RemoveFromProject(args) => {
            remove_from_project::run(ctx, &args).await
        }
    }
}
