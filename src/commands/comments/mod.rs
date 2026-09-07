//! 评论（comments）资源：`pc comments <operation>`。
//!
//! 评论是跨模块的通用资源（工作项、测试用例、需求、工单、Wiki 页面等主体均可评论），
//! 对应 `/v1/comments` 的 REST 接口，因此与工时一样直接挂在命令顶层。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件（如 `list.rs`），定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`CommentsCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::{Subcommand, ValueEnum};

use crate::commands::Ctx;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;

use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use list::ListArgs;

/// 评论主体类型（查询/请求体参数 `principal_type`）。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum PrincipalType {
    /// Project management work item
    Workitem,
    /// Work item review
    WorkitemReview,
    /// Testhub test run
    Testrun,
    /// Testhub test case
    Testcase,
    /// Test case review
    TestcaseReview,
    /// Ship idea (requirement)
    Idea,
    /// Idea review
    IdeaReview,
    /// Ship ticket
    Ticket,
    /// Wiki page
    Page,
}

impl PrincipalType {
    /// 序列化为接口要求的 snake_case 字符串。
    pub fn as_str(self) -> &'static str {
        match self {
            PrincipalType::Workitem => "workitem",
            PrincipalType::WorkitemReview => "workitem_review",
            PrincipalType::Testrun => "testrun",
            PrincipalType::Testcase => "testcase",
            PrincipalType::TestcaseReview => "testcase_review",
            PrincipalType::Idea => "idea",
            PrincipalType::IdeaReview => "idea_review",
            PrincipalType::Ticket => "ticket",
            PrincipalType::Page => "page",
        }
    }
}

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

    /// Get a comment by id (GET /v1/comments/{comment_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getCommentsByCommentIdByPrincipalTypeAndPrincipalId
    Get(GetArgs),

    /// Create a comment on a principal (POST /v1/comments)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postComments
    Create(CreateArgs),

    /// Delete a comment by id (DELETE /v1/comments/{comment_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteCommentsByCommentIdByPrincipalTypeAndPrincipalId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: CommentsCommand) -> anyhow::Result<()> {
    match command {
        CommentsCommand::List(args) => list::run(ctx, &args).await,
        CommentsCommand::Get(args) => get::run(ctx, &args).await,
        CommentsCommand::Create(args) => create::run(ctx, &args).await,
        CommentsCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
