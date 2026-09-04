//! 安全（security）模块：`pc security <resource> <operation>`。
//!
//! 对应 REST 路径前缀 `/v1/security/...`（开放平台文档中的「全局 › 安全」分组）：
//! 登录日志（login-log）、审计日志（audit-log）。
//!
//! 新增资源（resource）：
//! 1. 在 `src/commands/security/` 下新建资源目录（如 `login_log/`），
//!    在其中按操作（operation）拆分文件；
//! 2. 在本文件的 [`SecurityCommand`] 枚举加一个变体，
//!    并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod audit_log;
pub mod login_log;

use audit_log::AuditLogCommand;
use login_log::LoginLogCommand;

/// `pc security` 的资源级子命令。
#[derive(Debug, Subcommand)]
pub enum SecurityCommand {
    /// Login logs (sign-in records)
    LoginLog {
        #[command(subcommand)]
        command: LoginLogCommand,
    },
    /// Audit logs (operation records)
    AuditLog {
        #[command(subcommand)]
        command: AuditLogCommand,
    },
}

pub async fn run(ctx: &Ctx, command: SecurityCommand) -> anyhow::Result<()> {
    match command {
        SecurityCommand::LoginLog { command } => login_log::run(ctx, command).await,
        SecurityCommand::AuditLog { command } => audit_log::run(ctx, command).await,
    }
}
