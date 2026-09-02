//! 迭代（sprint）资源：`pc pjm sprint <operation>`。
//!
//! 对应 `/v1/pjm/projects/{project_id}/sprints`、`/v1/pjm/sprints/bulk`
//! 及相关直接子路径的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`SprintCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod bulk_create;
pub mod create;
pub mod get;
pub mod list;
pub mod update;

use bulk_create::BulkCreateArgs;
use create::CreateArgs;
use get::GetArgs;
use list::ListArgs;
use update::UpdateArgs;

/// `pc pjm sprint` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum SprintCommand {
    /// List sprints of a project (GET /v1/pjm/projects/{project_id}/sprints)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdSprints
    List(ListArgs),

    /// Get a sprint by id (GET /v1/pjm/projects/{project_id}/sprints/{sprint_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdSprintsBySprintId
    Get(GetArgs),

    /// Create a sprint (POST /v1/pjm/projects/{project_id}/sprints)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdSprints
    Create(CreateArgs),

    /// Partially update a sprint (PATCH /v1/pjm/projects/{project_id}/sprints/{sprint_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdSprintsBySprintId
    Update(UpdateArgs),

    /// Bulk create sprints across projects (POST /v1/pjm/sprints/bulk, enterprise token only)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmSprintsBulk
    BulkCreate(BulkCreateArgs),
}

pub async fn run(ctx: &Ctx, command: SprintCommand) -> anyhow::Result<()> {
    match command {
        SprintCommand::List(args) => list::run(ctx, &args).await,
        SprintCommand::Get(args) => get::run(ctx, &args).await,
        SprintCommand::Create(args) => create::run(ctx, &args).await,
        SprintCommand::Update(args) => update::run(ctx, &args).await,
        SprintCommand::BulkCreate(args) => bulk_create::run(ctx, &args).await,
    }
}
