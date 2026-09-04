//! 附件（attachments）资源：`pc attachments <operation>`。
//!
//! 附件是跨模块的通用资源（工作项、测试用例/测试计划、需求、工单、Wiki 页面、
//! 工作项交付目标等主体均可挂附件），对应 `/v1/attachments` 的 REST 接口，
//! 因此与评论、工时一样直接挂在命令顶层。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件（如 `list.rs`），定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`AttachmentsCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::{Subcommand, ValueEnum};

use crate::commands::Ctx;

pub mod delete;
pub mod get;
pub mod list;
pub mod upload_file;
pub mod upload_snippet;

use delete::DeleteArgs;
use get::GetArgs;
use list::ListArgs;
use upload_file::UploadFileArgs;
use upload_snippet::UploadSnippetArgs;

/// 附件主体类型（查询/请求体参数 `principal_type`）。
///
/// 各端点支持的主体集合略有差异（代码段上传不支持 `workitem_deliverable`），
/// 统一取并集，由服务端校验不支持的组合。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum PrincipalType {
    /// Project management work item
    Workitem,
    /// Work item review
    WorkitemReview,
    /// Work item deliverable target
    WorkitemDeliverable,
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
            PrincipalType::WorkitemDeliverable => "workitem_deliverable",
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

/// `pc attachments` 的操作级子命令。
///
/// 操作级变体直接持有参数结构体（实现 `clap::Args`），
/// 不再有下一级子命令。
#[derive(Debug, Subcommand)]
pub enum AttachmentsCommand {
    /// List attachments of a principal (GET /v1/attachments)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getAttachmentsByPrincipalTypeAndPrincipalId
    List(ListArgs),

    /// Get an attachment by id (GET /v1/attachments/{attachment_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getAttachmentsByAttachmentId
    Get(GetArgs),

    /// Upload a file attachment (multipart POST /v1/attachments)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postAttachmentsByPrincipalTypeAndPrincipalId
    UploadFile(UploadFileArgs),

    /// Upload a code snippet attachment (POST /v1/attachments)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postAttachments
    UploadSnippet(UploadSnippetArgs),

    /// Delete an attachment (DELETE /v1/attachments/{attachment_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteAttachmentsByAttachmentIdByPrincipalTypeAndPrincipalId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: AttachmentsCommand) -> anyhow::Result<()> {
    match command {
        AttachmentsCommand::List(args) => list::run(ctx, &args).await,
        AttachmentsCommand::Get(args) => get::run(ctx, &args).await,
        AttachmentsCommand::UploadFile(args) => upload_file::run(ctx, &args).await,
        AttachmentsCommand::UploadSnippet(args) => upload_snippet::run(ctx, &args).await,
        AttachmentsCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
