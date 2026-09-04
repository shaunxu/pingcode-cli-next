//! 命令分发。
//!
//! 两类命令：
//! - **三级命令**：`pc <module> <resource> <operation>`（如 `pc pjm workitem create`）。
//!   每个模块一个目录（如 `pjm/`），模块内按资源建子目录，资源目录内按操作拆文件。
//! - **自由命令**：不遵循三级模式的命令（如 `state`），
//!   放在 `dynamic/` 下，每个命令一个文件，直接在本文件的 match 中分发。

pub mod context;
pub mod dynamic;
pub mod organization;
pub mod pjm;
pub mod security;
pub mod ship;
pub mod testhub;
pub mod wiki;
pub mod workload;
pub mod workload_type;

use context::Ctx;

use crate::cli::Command;
use crate::config::Config;

pub async fn run(command: Command, config: &Config) -> anyhow::Result<()> {
    let ctx = Ctx::new(config.clone()).await?;

    match command {
        // 三级命令：module -> resource -> operation
        Command::Organization {
            command: organization_command,
        } => organization::run(&ctx, organization_command).await,

        // 三级命令：module -> resource -> operation
        Command::Pjm {
            command: pjm_command,
        } => pjm::run(&ctx, pjm_command).await,

        // 三级命令：module -> resource -> operation
        Command::Ship {
            command: ship_command,
        } => ship::run(&ctx, ship_command).await,

        // 三级命令：module -> resource -> operation
        Command::Testhub {
            command: testhub_command,
        } => testhub::run(&ctx, testhub_command).await,

        // 三级命令：module -> resource -> operation
        Command::Wiki {
            command: wiki_command,
        } => wiki::run(&ctx, wiki_command).await,

        // 三级命令：module -> resource -> operation
        Command::Security {
            command: security_command,
        } => security::run(&ctx, security_command).await,

        // 跨模块全局资源：工时直接挂在顶层（pc workload <operation>）
        Command::Workload {
            command: workload_command,
        } => workload::run(&ctx, workload_command).await,

        // 跨模块全局资源：工时类型直接挂在顶层（pc workload-type <operation>）
        Command::WorkloadType {
            command: workload_type_command,
        } => workload_type::run(&ctx, workload_type_command).await,

        // 自由命令：不遵循 module/resource/operation 模式
        Command::State => dynamic::state::run(&ctx).await,
    }
}
