//! 评论（comments）资源：`pc comments <operation>`。
//!
//! 评论是跨模块的通用资源（工作项、测试用例、需求、工单、Wiki 页面等主体均可评论），
//! 对应 `/v1/comments` 的 REST 接口，因此与工时一样直接挂在命令顶层。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件（如 `list.rs`），定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`CommentsCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod list;

use list::ListArgs;

/// `pc comments` 的操作级子命令。
///
/// 操作级变体直接持有参数结构体（实现 `clap::Args`），
/// 不再有下一级子命令。
#[derive(Debug, Subcommand)]
pub enum CommentsCommand {
    /// List comments of a principal (GET /v1/comments)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getCommentsByPrincipalTypeAndPrincipalId
    List(ListArgs),
}

pub async fn run(ctx: &Ctx, command: CommentsCommand) -> anyhow::Result<()> {
    match command {
        CommentsCommand::List(args) => list::run(ctx, &args).await,
    }
}
