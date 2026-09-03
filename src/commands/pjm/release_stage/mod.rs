//! 发布阶段（release stage）资源：`pc pjm release-stage <operation>`。
//!
//! 对应 `/v1/pjm/release_stages` 及其直接子路径的 REST 接口。
//! 发布阶段是企业级配置（不隶属于某个项目），scope 为
//! `pcp:(read|write):pjm:configuration`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ReleaseStageCommand`] 枚举加一个变体，并在 [`run`] 的 match 中
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

/// `pc pjm release-stage` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ReleaseStageCommand {
    /// List release stages of the enterprise (GET /v1/pjm/release_stages)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmReleaseStages
    List(ListArgs),

    /// Get a release stage by id (GET /v1/pjm/release_stages/{release_stage_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmReleaseStagesByReleaseStageId
    Get(GetArgs),

    /// Create a release stage (POST /v1/pjm/release_stages)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmReleaseStages
    Create(CreateArgs),

    /// Partially update a release stage (PATCH /v1/pjm/release_stages/{release_stage_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmReleaseStagesByReleaseStageId
    Update(UpdateArgs),

    /// Delete a release stage (DELETE /v1/pjm/release_stages/{release_stage_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmReleaseStagesByReleaseStageId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: ReleaseStageCommand) -> anyhow::Result<()> {
    match command {
        ReleaseStageCommand::List(args) => list::run(ctx, &args).await,
        ReleaseStageCommand::Get(args) => get::run(ctx, &args).await,
        ReleaseStageCommand::Create(args) => create::run(ctx, &args).await,
        ReleaseStageCommand::Update(args) => update::run(ctx, &args).await,
        ReleaseStageCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
