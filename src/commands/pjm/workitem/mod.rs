//! 工作项（workitem）资源：`pc pjm workitem <operation>`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件（如 `create.rs`），定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;

use create::CreateArgs;

/// `pc pjm workitem` 的操作级子命令。
///
/// 操作级变体直接持有参数结构体（实现 `clap::Args`），
/// 不再有下一级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkitemCommand {
    /// Create a work item (POST /v1/pjm/workitems)
    Create(CreateArgs),
}

pub async fn run(ctx: &Ctx, command: WorkitemCommand) -> anyhow::Result<()> {
    match command {
        WorkitemCommand::Create(args) => create::run(ctx, &args).await,
    }
}
