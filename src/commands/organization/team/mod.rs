//! 企业（team）资源：`pc organization team <operation>`。
//!
//! 对应 `/v1/directory/team` 的 REST 接口。企业是单例资源，仅支持获取。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TeamCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;

use get::GetArgs;

/// `pc organization team` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TeamCommand {
    /// Get the current enterprise/team info (GET /v1/directory/team)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryTeam
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: TeamCommand) -> anyhow::Result<()> {
    match command {
        TeamCommand::Get(args) => get::run(ctx, &args).await,
    }
}
