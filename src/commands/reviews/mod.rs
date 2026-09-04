//! 评审（reviews）资源：`pc reviews <operation>`。
//!
//! 评审是跨模块的通用资源（工作项、测试用例、需求均可发起评审），对应
//! `/v1/reviews` 的 REST 接口，因此与评论、工时一样直接挂在命令顶层。
//!
//! 评审本身与"评审内容"（被评审的工作项/需求/用例）是两级资源：评审内容挂在
//! `/v1/reviews/{review_id}/principals` 下，本模块用 `*-principal` 操作
//! （list-principal / add-principal / get-principal / remove-principal）表达。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件（如 `list.rs`），定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ReviewsCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::{Subcommand, ValueEnum};

use crate::commands::Ctx;

pub mod add_principal;
pub mod create;
pub mod delete;
pub mod get;
pub mod get_principal;
pub mod list;
pub mod list_principals;
pub mod remove_principal;

use add_principal::AddPrincipalArgs;
use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use get_principal::GetPrincipalArgs;
use list::ListArgs;
use list_principals::ListPrincipalsArgs;
use remove_principal::RemovePrincipalArgs;

/// 评审主体类型（查询/请求体参数 `principal_type`）。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum PrincipalType {
    /// Project management work item
    Workitem,
    /// Testhub test case
    Testcase,
    /// Ship idea (requirement)
    Idea,
}

impl PrincipalType {
    /// 序列化为接口要求的 snake_case 字符串。
    pub fn as_str(self) -> &'static str {
        match self {
            PrincipalType::Workitem => "workitem",
            PrincipalType::Testcase => "testcase",
            PrincipalType::Idea => "idea",
        }
    }
}

/// 评审状态（查询参数 `status`）。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum ReviewStatus {
    /// Pending review
    Pending,
    /// Review in progress
    InProgress,
    /// Review completed
    Completed,
    /// Review repealed (withdrawn)
    Repealed,
}

impl ReviewStatus {
    /// 序列化为接口要求的 snake_case 字符串。
    pub fn as_str(self) -> &'static str {
        match self {
            ReviewStatus::Pending => "pending",
            ReviewStatus::InProgress => "in_progress",
            ReviewStatus::Completed => "completed",
            ReviewStatus::Repealed => "repealed",
        }
    }
}

/// `pc reviews` 的操作级子命令。
///
/// 操作级变体直接持有参数结构体（实现 `clap::Args`），
/// 不再有下一级子命令。
#[derive(Debug, Subcommand)]
pub enum ReviewsCommand {
    /// List reviews in a project, product or test library (GET /v1/reviews)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getReviewsByPrincipalTypeAndPilotId
    List(Box<ListArgs>),

    /// Get a review by id (GET /v1/reviews/{review_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getReviewsByReviewId
    Get(GetArgs),

    /// Create a review (POST /v1/reviews)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postReviews
    Create(CreateArgs),

    /// Delete a review by id (DELETE /v1/reviews/{review_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteReviewsByReviewIdByPrincipalType
    Delete(DeleteArgs),

    /// List principals (reviewed items) of a review (GET /v1/reviews/{review_id}/principals)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getReviewsByReviewIdPrincipalsByPrincipalType
    ListPrincipals(ListPrincipalsArgs),

    /// Add a principal (reviewed item) to a review (POST /v1/reviews/{review_id}/principals)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postReviewsByReviewIdPrincipals
    AddPrincipal(AddPrincipalArgs),

    /// Get a principal of a review (GET /v1/reviews/{review_id}/principals/{principal_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getReviewsByReviewIdPrincipalsByPrincipalIdByPrincipalType
    GetPrincipal(GetPrincipalArgs),

    /// Remove a principal from a review (DELETE /v1/reviews/{review_id}/principals/{principal_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteReviewsByReviewIdPrincipalsByPrincipalIdByPrincipalType
    RemovePrincipal(RemovePrincipalArgs),
}

pub async fn run(ctx: &Ctx, command: ReviewsCommand) -> anyhow::Result<()> {
    match command {
        ReviewsCommand::List(args) => list::run(ctx, &args).await,
        ReviewsCommand::Get(args) => get::run(ctx, &args).await,
        ReviewsCommand::Create(args) => create::run(ctx, &args).await,
        ReviewsCommand::Delete(args) => delete::run(ctx, &args).await,
        ReviewsCommand::ListPrincipals(args) => list_principals::run(ctx, &args).await,
        ReviewsCommand::AddPrincipal(args) => add_principal::run(ctx, &args).await,
        ReviewsCommand::GetPrincipal(args) => get_principal::run(ctx, &args).await,
        ReviewsCommand::RemovePrincipal(args) => remove_principal::run(ctx, &args).await,
    }
}
