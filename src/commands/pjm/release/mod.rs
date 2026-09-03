//! 发布（release）资源：`pc pjm release <operation>`。
//!
//! 对应 `/v1/pjm/projects/{project_id}/releases`、`/v1/pjm/releases/bulk`
//! 及相关直接子路径的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ReleaseCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod bulk_create;
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

use bulk_create::BulkCreateArgs;
use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use list::ListArgs;
use update::UpdateArgs;

/// `pc pjm release` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ReleaseCommand {
    /// List releases of a project (GET /v1/pjm/projects/{project_id}/releases)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdReleases
    List(ListArgs),

    /// Get a release by id (GET /v1/pjm/projects/{project_id}/releases/{release_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdReleasesByReleaseId
    Get(GetArgs),

    /// Create a release (POST /v1/pjm/projects/{project_id}/releases)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdReleases
    Create(CreateArgs),

    /// Partially update a release (PATCH /v1/pjm/projects/{project_id}/releases/{release_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdReleasesByReleaseId
    Update(UpdateArgs),

    /// Delete a release (DELETE /v1/pjm/projects/{project_id}/releases/{release_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdReleasesByReleaseId
    Delete(DeleteArgs),

    /// Bulk create releases across projects (POST /v1/pjm/releases/bulk, enterprise token only)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmReleasesBulk
    BulkCreate(BulkCreateArgs),
}

pub async fn run(ctx: &Ctx, command: ReleaseCommand) -> anyhow::Result<()> {
    match command {
        ReleaseCommand::List(args) => list::run(ctx, &args).await,
        ReleaseCommand::Get(args) => get::run(ctx, &args).await,
        ReleaseCommand::Create(args) => create::run(ctx, &args).await,
        ReleaseCommand::Update(args) => update::run(ctx, &args).await,
        ReleaseCommand::Delete(args) => delete::run(ctx, &args).await,
        ReleaseCommand::BulkCreate(args) => bulk_create::run(ctx, &args).await,
    }
}
