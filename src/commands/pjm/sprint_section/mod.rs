//! 迭代分组（sprint section）资源：`pc pjm sprint-section <operation>`。
//!
//! 对应 `/v1/pjm/projects/{project_id}/sprint_sections` 及其直接子路径的
//! REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`SprintSectionCommand`] 枚举加一个变体，并在 [`run`] 的 match 中
//!    加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use list::ListArgs;
use update::UpdateArgs;

/// `pc pjm sprint-section` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum SprintSectionCommand {
    /// List sprint sections of a project (GET /v1/pjm/projects/{project_id}/sprint_sections)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdSprintSections
    List(ListArgs),

    /// Get a sprint section by id (GET /v1/pjm/projects/{project_id}/sprint_sections/{section_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdSprintSectionsBySectionId
    Get(GetArgs),

    /// Create a sprint section (POST /v1/pjm/projects/{project_id}/sprint_sections)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdSprintSections
    Create(CreateArgs),

    /// Partially update a sprint section (PATCH /v1/pjm/projects/{project_id}/sprint_sections/{section_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdSprintSectionsBySectionId
    Update(UpdateArgs),

    /// Delete a sprint section (DELETE /v1/pjm/projects/{project_id}/sprint_sections/{section_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdSprintSectionsBySectionId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: SprintSectionCommand) -> anyhow::Result<()> {
    match command {
        SprintSectionCommand::List(args) => list::run(ctx, &args).await,
        SprintSectionCommand::Get(args) => get::run(ctx, &args).await,
        SprintSectionCommand::Create(args) => create::run(ctx, &args).await,
        SprintSectionCommand::Update(args) => update::run(ctx, &args).await,
        SprintSectionCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
