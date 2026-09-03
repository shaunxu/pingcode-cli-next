//! 发布类别（release category）资源：`pc pjm release-category <operation>`。
//!
//! 对应 `/v1/pjm/projects/{project_id}/release_categories` 及其直接子路径的
//! REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ReleaseCategoryCommand`] 枚举加一个变体，并在 [`run`] 的 match 中
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

/// `pc pjm release-category` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ReleaseCategoryCommand {
    /// List release categories of a project (GET /v1/pjm/projects/{project_id}/release_categories)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdReleaseCategories
    List(ListArgs),

    /// Get a release category by id (GET /v1/pjm/projects/{project_id}/release_categories/{release_category_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdReleaseCategoriesByReleaseCategoryId
    Get(GetArgs),

    /// Create a release category (POST /v1/pjm/projects/{project_id}/release_categories)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdReleaseCategories
    Create(CreateArgs),

    /// Partially update a release category (PATCH /v1/pjm/projects/{project_id}/release_categories/{release_category_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdReleaseCategoriesByReleaseCategoryId
    Update(UpdateArgs),

    /// Delete a release category (DELETE /v1/pjm/projects/{project_id}/release_categories/{release_category_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdReleaseCategoriesByReleaseCategoryId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: ReleaseCategoryCommand) -> anyhow::Result<()> {
    match command {
        ReleaseCategoryCommand::List(args) => list::run(ctx, &args).await,
        ReleaseCategoryCommand::Get(args) => get::run(ctx, &args).await,
        ReleaseCategoryCommand::Create(args) => create::run(ctx, &args).await,
        ReleaseCategoryCommand::Update(args) => update::run(ctx, &args).await,
        ReleaseCategoryCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
