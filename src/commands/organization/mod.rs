//! 组织（organization）模块：`pc organization <resource> <operation>`。
//!
//! 对应 REST 路径前缀 `/v1/directory/...`（开放平台文档中的「全局 › 组织」分组）：
//! 企业（team）、企业成员（user）、部门（department）、团队（group）、
//! 团队成员（group-member）、角色（role）、职位（job）。
//!
//! 新增资源（resource）：
//! 1. 在 `src/commands/organization/` 下新建资源目录（如 `user/`），
//!    在其中按操作（operation）拆分文件；
//! 2. 在本文件的 [`OrganizationCommand`] 枚举加一个变体，
//!    并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod department;
pub mod group;
pub mod group_member;
pub mod job;
pub mod role;
pub mod team;
pub mod user;

use department::DepartmentCommand;
use group::GroupCommand;
use group_member::GroupMemberCommand;
use job::JobCommand;
use role::RoleCommand;
use team::TeamCommand;
use user::UserCommand;

/// `pc organization` 的资源级子命令。
#[derive(Debug, Subcommand)]
pub enum OrganizationCommand {
    /// Enterprise (team) singleton info
    Team {
        #[command(subcommand)]
        command: TeamCommand,
    },
    /// Enterprise members (users)
    User {
        #[command(subcommand)]
        command: UserCommand,
    },
    /// Departments
    Department {
        #[command(subcommand)]
        command: DepartmentCommand,
    },
    /// Teams (user groups)
    Group {
        #[command(subcommand)]
        command: GroupCommand,
    },
    /// Team members
    GroupMember {
        #[command(subcommand)]
        command: GroupMemberCommand,
    },
    /// Enterprise roles (read-only)
    Role {
        #[command(subcommand)]
        command: RoleCommand,
    },
    /// Enterprise jobs / job titles (read-only)
    Job {
        #[command(subcommand)]
        command: JobCommand,
    },
}

pub async fn run(ctx: &Ctx, command: OrganizationCommand) -> anyhow::Result<()> {
    match command {
        OrganizationCommand::Team { command } => team::run(ctx, command).await,
        OrganizationCommand::User { command } => user::run(ctx, command).await,
        OrganizationCommand::Department { command } => department::run(ctx, command).await,
        OrganizationCommand::Group { command } => group::run(ctx, command).await,
        OrganizationCommand::GroupMember { command } => group_member::run(ctx, command).await,
        OrganizationCommand::Role { command } => role::run(ctx, command).await,
        OrganizationCommand::Job { command } => job::run(ctx, command).await,
    }
}
