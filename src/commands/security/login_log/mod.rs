//! 登录日志（login-log）资源：`pc security login-log <operation>`。
//!
//! 对应 `/v1/security/login_logs` 的 REST 接口。只读。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`LoginLogCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod list;

use list::ListArgs;

/// `pc security login-log` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum LoginLogCommand {
    /// List login logs (GET /v1/security/login_logs)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getSecurityLoginLogs
    List(Box<ListArgs>),
}

pub async fn run(ctx: &Ctx, command: LoginLogCommand) -> anyhow::Result<()> {
    match command {
        LoginLogCommand::List(args) => list::run(ctx, &args).await,
    }
}
