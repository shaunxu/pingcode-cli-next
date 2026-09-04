use clap::Args;
use serde_json::{json, Value};

use crate::commands::attachments::PrincipalType;
use crate::commands::Ctx;
use crate::output;

/// `pc attachments list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Type of the principal the attachments belong to
    #[arg(long, value_enum, value_name = "TYPE")]
    pub principal_type: PrincipalType,

    /// Id of the principal (work item, test run, idea, ticket, page, ...)
    #[arg(long, value_name = "ID")]
    pub principal_id: String,

    /// Comment id; pass to list attachments of a comment on the principal
    #[arg(long, value_name = "ID")]
    pub comment_id: Option<String>,

    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 获取附件列表：`GET /v1/attachments`（分页，scope 依赖附件所属主体，
/// 如 workitem 需要 `pcp:read:pjm:workitem`）。
///
/// 查询参数：
/// - `principal_type`：附件主体类型（`workitem`/`workitem_review`/
///   `workitem_deliverable`/`testrun`/`testcase`/`testcase_review`/`idea`/
///   `idea_review`/`ticket`/`page`）；
/// - `principal_id`：附件主体 id；
/// - `comment_id`：获取评论附件时传评论 id；
/// - `page_index`/`page_size`：分页参数。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getAttachmentsByPrincipalTypeAndPrincipalId
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("principal_type".into(), json!(args.principal_type.as_str()));
    query.insert("principal_id".into(), json!(args.principal_id));
    if let Some(comment_id) = &args.comment_id {
        query.insert("comment_id".into(), json!(comment_id));
    }
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), json!(page_size));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/attachments", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
