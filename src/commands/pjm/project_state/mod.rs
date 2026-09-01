//! 项目状态（project state）资源：`pc pjm project-state <operation>`。
//!
//! 对应 `/v1/pjm/project/states` 与 `/v1/pjm/project_states` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ProjectStateCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc pjm project-state` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ProjectStateCommand {
    /// List project states (GET /v1/pjm/project/states?project_id={project_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectStatesByProjectId
    List(ListArgs),

    /// Get a project state by id (GET /v1/pjm/project_states/{state_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectStatesByStateId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: ProjectStateCommand) -> anyhow::Result<()> {
    match command {
        ProjectStateCommand::List(args) => list::run(ctx, &args).await,
        ProjectStateCommand::Get(args) => get::run(ctx, &args).await,
    }
}
