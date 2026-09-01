//! 项目（project）资源：`pc pjm project <operation>`。
//!
//! 对应 `/v1/pjm/projects` 及其直接子路径的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ProjectCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod clone;
pub mod create;
pub mod enable_local_config;
pub mod get;
pub mod list;
pub mod progress;
pub mod update;

use clone::CloneArgs;
use create::CreateArgs;
use enable_local_config::EnableLocalConfigArgs;
use get::GetArgs;
use list::ListArgs;
use progress::ProgressArgs;
use update::UpdateArgs;

/// `pc pjm project` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ProjectCommand {
    /// List projects (GET /v1/pjm/projects)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjects
    List(ListArgs),

    /// Get a project by id (GET /v1/pjm/projects/{project_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectId
    Get(GetArgs),

    /// Create a project (POST /v1/pjm/projects)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjects
    Create(CreateArgs),

    /// Partially update a project (PATCH /v1/pjm/projects/{project_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectId
    Update(UpdateArgs),

    /// Clone (copy) a project (POST /v1/pjm/projects/{project_id}/clone)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdClone
    Clone(CloneArgs),

    /// Enable local config for a project (POST /v1/pjm/projects/{project_id}/local_config/enable)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdLocalConfigEnable
    EnableLocalConfig(EnableLocalConfigArgs),

    /// Get a project's progress (GET /v1/pjm/projects/{project_id}/progress)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdProgress
    Progress(ProgressArgs),
}

pub async fn run(ctx: &Ctx, command: ProjectCommand) -> anyhow::Result<()> {
    match command {
        ProjectCommand::List(args) => list::run(ctx, &args).await,
        ProjectCommand::Get(args) => get::run(ctx, &args).await,
        ProjectCommand::Create(args) => create::run(ctx, &args).await,
        ProjectCommand::Update(args) => update::run(ctx, &args).await,
        ProjectCommand::Clone(args) => clone::run(ctx, &args).await,
        ProjectCommand::EnableLocalConfig(args) => enable_local_config::run(ctx, &args).await,
        ProjectCommand::Progress(args) => progress::run(ctx, &args).await,
    }
}
