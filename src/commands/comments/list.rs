use clap::{Args, ValueEnum};
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// 评论主体类型（查询参数 `principal_type`）。
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
    fn as_str(self) -> &'static str {
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

/// `pc comments list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Type of the principal the comments belong to
    #[arg(long, value_enum, value_name = "TYPE")]
    pub principal_type: PrincipalType,

    /// Id of the principal (work item, test run, idea, ticket, page, ...)
    #[arg(long, value_name = "ID")]
    pub principal_id: String,

    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 获取评论列表：`GET /v1/comments`（分页，scope 依赖评论所属主体，
/// 如 workitem 需要 `pcp:read:pjm:workitem`）。
///
/// 查询参数：
/// - `principal_type`：评论主体类型（`workitem`/`workitem_review`/`testrun`/
///   `testcase`/`testcase_review`/`idea`/`idea_review`/`ticket`/`page`）；
/// - `principal_id`：评论主体 id；
/// - `page_index`/`page_size`：分页参数。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getCommentsByPrincipalTypeAndPrincipalId
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("principal_type".into(), json!(args.principal_type.as_str()));
    query.insert("principal_id".into(), json!(args.principal_id));
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), json!(page_size));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/comments", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
