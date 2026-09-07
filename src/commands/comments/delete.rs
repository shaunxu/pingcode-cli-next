use clap::Args;
use serde_json::{json, Value};

use crate::commands::comments::PrincipalType;
use crate::commands::Ctx;
use crate::output;

/// `pc comments delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Comment id
    #[arg(value_name = "COMMENT_ID")]
    pub comment_id: String,

    /// Type of the principal the comment belongs to
    #[arg(long, value_enum, value_name = "TYPE")]
    pub principal_type: PrincipalType,

    /// Id of the principal (work item, test run, idea, ticket, page, ...)
    #[arg(long, value_name = "ID")]
    pub principal_id: String,
}

/// 删除一条评论：`DELETE /v1/comments/{comment_id}`（scope 依赖评论所属主体，
/// 如 workitem 需要 `pcp:write:pjm:workitem`）。
///
/// 查询参数 `principal_type`（评论主体类型：`workitem`/`workitem_review`/
/// `testrun`/`testcase`/`testcase_review`/`idea`/`idea_review`/`ticket`/`page`）、
/// `principal_id`（评论主体 id）必填。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteCommentsByCommentIdByPrincipalTypeAndPrincipalId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("principal_type".into(), json!(args.principal_type.as_str()));
    query.insert("principal_id".into(), json!(&args.principal_id));

    let path = format!("/v1/comments/{}", args.comment_id);
    let response: Value = ctx
        .client
        .delete_with_query(&path, &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
