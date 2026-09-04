//! 关注人（participants）资源：`pc participants <operation>`。
//!
//! 关注人是跨模块的通用资源（工作项、测试用例、需求、工单、Wiki 页面等主体
//! 均可添加关注人，可以是用户或团队），对应 `/v1/participants` 的 REST 接口，
//! 因此与评论、工时一样直接挂在命令顶层。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件（如 `list.rs`），定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ParticipantsCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::{Subcommand, ValueEnum};

use crate::commands::Ctx;

pub mod add;
pub mod get;
pub mod list;
pub mod remove;

use add::AddArgs;
use get::GetArgs;
use list::ListArgs;
use remove::RemoveArgs;

/// 关注人主体类型（查询/请求体参数 `principal_type`）。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum PrincipalType {
    /// Project management work item
    Workitem,
    /// Work item review
    WorkitemReview,
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
            PrincipalType::Testcase => "testcase",
            PrincipalType::TestcaseReview => "testcase_review",
            PrincipalType::Idea => "idea",
            PrincipalType::IdeaReview => "idea_review",
            PrincipalType::Ticket => "ticket",
            PrincipalType::Page => "page",
        }
    }
}

/// `pc participants` 的操作级子命令。
///
/// 操作级变体直接持有参数结构体（实现 `clap::Args`），
/// 不再有下一级子命令。
#[derive(Debug, Subcommand)]
pub enum ParticipantsCommand {
    /// List participants of a principal (GET /v1/participants)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getParticipantsByPrincipalTypeAndPrincipalId
    List(ListArgs),

    /// Get a participant by id (GET /v1/participants/{participant_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getParticipantsByParticipantId
    Get(GetArgs),

    /// Add a participant to a principal (POST /v1/participants)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postParticipants
    Add(AddArgs),

    /// Remove a participant from a principal (DELETE /v1/participants/{participant_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteParticipantsByParticipantIdByPrincipalTypeAndPrincipalId
    Remove(RemoveArgs),
}

pub async fn run(ctx: &Ctx, command: ParticipantsCommand) -> anyhow::Result<()> {
    match command {
        ParticipantsCommand::List(args) => list::run(ctx, &args).await,
        ParticipantsCommand::Get(args) => get::run(ctx, &args).await,
        ParticipantsCommand::Add(args) => add::run(ctx, &args).await,
        ParticipantsCommand::Remove(args) => remove::run(ctx, &args).await,
    }
}
