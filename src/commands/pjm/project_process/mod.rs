//! 项目流程（project process）资源（只读）：
//! `pc pjm project-process <operation>`。
//!
//! 对应「项目配置」中的项目流程字典 `/v1/pjm/processes` 及其直接子路径的
//! REST 接口，scope 为 `pcp:(read|write):pjm:configuration`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ProjectProcessCommand`] 枚举加一个变体，并在 [`run`] 的 match
//!    中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc pjm project-process` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ProjectProcessCommand {
    /// List all project processes (GET /v1/pjm/processes)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProcesses
    List(ListArgs),

    /// Get a project process by id (GET /v1/pjm/processes/{process_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProcessesByProcessId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: ProjectProcessCommand) -> anyhow::Result<()> {
    match command {
        ProjectProcessCommand::List(args) => list::run(ctx, &args).await,
        ProjectProcessCommand::Get(args) => get::run(ctx, &args).await,
    }
}
