//! 审计日志（audit-log）资源：`pc security audit-log <operation>`。
//!
//! 对应 `/v1/security/audit_logs` 的 REST 接口。只读。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`AuditLogCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod list;

use list::ListArgs;

/// `pc security audit-log` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum AuditLogCommand {
    /// List audit logs (GET /v1/security/audit_logs)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getSecurityAuditLogs
    List(Box<ListArgs>),
}

pub async fn run(ctx: &Ctx, command: AuditLogCommand) -> anyhow::Result<()> {
    match command {
        AuditLogCommand::List(args) => list::run(ctx, &args).await,
    }
}
