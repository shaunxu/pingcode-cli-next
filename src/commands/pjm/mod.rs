//! PJM（项目管理）模块：`pc pjm <resource> <operation>`。
//!
//! 新增资源（resource）：
//! 1. 在 `src/commands/pjm/` 下新建资源目录（如 `workitem/`），
//!    在其中按操作（operation）拆分文件；
//! 2. 在本文件的 [`PjmCommand`] 枚举加一个变体，
//!    并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod project;
pub mod project_member;
pub mod project_property;
pub mod project_state;
pub mod sprint;
pub mod sprint_category;
pub mod sprint_section;
pub mod workitem;

use project::ProjectCommand;
use project_member::ProjectMemberCommand;
use project_property::ProjectPropertyCommand;
use project_state::ProjectStateCommand;
use sprint::SprintCommand;
use sprint_category::SprintCategoryCommand;
use sprint_section::SprintSectionCommand;
use workitem::WorkitemCommand;

/// `pc pjm` 的资源级子命令。
#[derive(Debug, Subcommand)]
pub enum PjmCommand {
    /// Projects
    Project {
        #[command(subcommand)]
        command: ProjectCommand,
    },
    /// Project members
    ProjectMember {
        #[command(subcommand)]
        command: ProjectMemberCommand,
    },
    /// Project properties (global definitions and per-project configuration)
    ProjectProperty {
        #[command(subcommand)]
        command: ProjectPropertyCommand,
    },
    /// Project states
    ProjectState {
        #[command(subcommand)]
        command: ProjectStateCommand,
    },
    /// Sprints (iterations)
    Sprint {
        #[command(subcommand)]
        command: SprintCommand,
    },
    /// Sprint sections (iteration groups)
    SprintSection {
        #[command(subcommand)]
        command: SprintSectionCommand,
    },
    /// Sprint categories (iteration categories)
    SprintCategory {
        #[command(subcommand)]
        command: SprintCategoryCommand,
    },
    /// Work items (requirements, tasks, bugs, ...)
    Workitem {
        #[command(subcommand)]
        command: WorkitemCommand,
    },
}

pub async fn run(ctx: &Ctx, command: PjmCommand) -> anyhow::Result<()> {
    match command {
        PjmCommand::Project { command } => project::run(ctx, command).await,
        PjmCommand::ProjectMember { command } => project_member::run(ctx, command).await,
        PjmCommand::ProjectProperty { command } => project_property::run(ctx, command).await,
        PjmCommand::ProjectState { command } => project_state::run(ctx, command).await,
        PjmCommand::Sprint { command } => sprint::run(ctx, command).await,
        PjmCommand::SprintSection { command } => sprint_section::run(ctx, command).await,
        PjmCommand::SprintCategory { command } => sprint_category::run(ctx, command).await,
        PjmCommand::Workitem { command } => workitem::run(ctx, command).await,
    }
}
