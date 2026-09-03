//! Wiki（知识管理）模块：`pc wiki <resource> <operation>`。
//!
//! 新增资源（resource）：
//! 1. 在 `src/commands/wiki/` 下新建资源目录（如 `space/`），
//!    在其中按操作（operation）拆分文件；
//! 2. 在本文件的 [`WikiCommand`] 枚举加一个变体，
//!    并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod page;
pub mod space;
pub mod space_member;

use page::PageCommand;
use space::SpaceCommand;
use space_member::SpaceMemberCommand;

/// `pc wiki` 的资源级子命令。
#[derive(Debug, Subcommand)]
pub enum WikiCommand {
    /// Wiki spaces (knowledge bases)
    Space {
        #[command(subcommand)]
        command: SpaceCommand,
    },
    /// Wiki space members
    SpaceMember {
        #[command(subcommand)]
        command: SpaceMemberCommand,
    },
    /// Wiki pages (documents)
    Page {
        #[command(subcommand)]
        command: PageCommand,
    },
}

pub async fn run(ctx: &Ctx, command: WikiCommand) -> anyhow::Result<()> {
    match command {
        WikiCommand::Space { command } => space::run(ctx, command).await,
        WikiCommand::SpaceMember { command } => space_member::run(ctx, command).await,
        WikiCommand::Page { command } => page::run(ctx, command).await,
    }
}
