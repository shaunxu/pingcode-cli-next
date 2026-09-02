//! 迭代类别（sprint category）资源：`pc pjm sprint-category <operation>`。
//!
//! 对应 `/v1/pjm/projects/{project_id}/sprint_categories` 及其直接子路径的
//! REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`SprintCategoryCommand`] 枚举加一个变体，并在 [`run`] 的 match 中
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

/// `pc pjm sprint-category` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum SprintCategoryCommand {
    /// List sprint categories of a project (GET /v1/pjm/projects/{project_id}/sprint_categories)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdSprintCategories
    List(ListArgs),

    /// Get a sprint category by id (GET /v1/pjm/projects/{project_id}/sprint_categories/{sprint_category_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdSprintCategoriesBySprintCategoryId
    Get(GetArgs),

    /// Create a sprint category (POST /v1/pjm/projects/{project_id}/sprint_categories)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdSprintCategories
    Create(CreateArgs),

    /// Partially update a sprint category (PATCH /v1/pjm/projects/{project_id}/sprint_categories/{sprint_category_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdSprintCategoriesBySprintCategoryId
    Update(UpdateArgs),

    /// Delete a sprint category (DELETE /v1/pjm/projects/{project_id}/sprint_categories/{sprint_category_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdSprintCategoriesBySprintCategoryId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: SprintCategoryCommand) -> anyhow::Result<()> {
    match command {
        SprintCategoryCommand::List(args) => list::run(ctx, &args).await,
        SprintCategoryCommand::Get(args) => get::run(ctx, &args).await,
        SprintCategoryCommand::Create(args) => create::run(ctx, &args).await,
        SprintCategoryCommand::Update(args) => update::run(ctx, &args).await,
        SprintCategoryCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
