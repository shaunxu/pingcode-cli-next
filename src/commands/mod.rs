//! 命令分发。
//!
//! 两类命令：
//! - **三级命令**：`pc <module> <resource> <operation>`（如 `pc pjm workitem create`）。
//!   每个模块一个目录（如 `pjm/`），模块内按资源建子目录，资源目录内按操作拆文件。
//! - **自由命令**：不遵循三级模式的命令（如 `state`、`whoami`），
//!   放在 `free/` 下，每个命令一个文件，直接在本文件的 match 中分发。

pub mod context;
pub mod free;
pub mod pjm;

use context::Ctx;

use crate::cli::Command;
use crate::config::Config;

pub async fn run(command: Command, config: &Config) -> anyhow::Result<()> {
    let ctx = Ctx::new(config.clone()).await?;

    match command {
        // 三级命令：module -> resource -> operation
        Command::Pjm {
            command: pjm_command,
        } => pjm::run(&ctx, pjm_command).await,

        // 自由命令：不遵循 module/resource/operation 模式
        Command::State => free::state::run(&ctx).await,
        Command::Whoami => free::whoami::run(&ctx).await,
    }
}
