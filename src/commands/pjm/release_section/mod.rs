//! 发布分组（release section）资源：`pc pjm release-section <operation>`。
//!
//! 对应 `/v1/pjm/projects/{project_id}/release_sections` 及其直接子路径的
//! REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ReleaseSectionCommand`] 枚举加一个变体，并在 [`run`] 的 match 中
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

/// `pc pjm release-section` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ReleaseSectionCommand {
    /// List release sections of a project (GET /v1/pjm/projects/{project_id}/release_sections)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdReleaseSections
    List(ListArgs),

    /// Get a release section by id (GET /v1/pjm/projects/{project_id}/release_sections/{release_section_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdReleaseSectionsByReleaseSectionId
    Get(GetArgs),

    /// Create a release section (POST /v1/pjm/projects/{project_id}/release_sections)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdReleaseSections
    Create(CreateArgs),

    /// Partially update a release section (PATCH /v1/pjm/projects/{project_id}/release_sections/{release_section_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdReleaseSectionsByReleaseSectionId
    Update(UpdateArgs),

    /// Delete a release section (DELETE /v1/pjm/projects/{project_id}/release_sections/{release_section_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdReleaseSectionsByReleaseSectionId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: ReleaseSectionCommand) -> anyhow::Result<()> {
    match command {
        ReleaseSectionCommand::List(args) => list::run(ctx, &args).await,
        ReleaseSectionCommand::Get(args) => get::run(ctx, &args).await,
        ReleaseSectionCommand::Create(args) => create::run(ctx, &args).await,
        ReleaseSectionCommand::Update(args) => update::run(ctx, &args).await,
        ReleaseSectionCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
