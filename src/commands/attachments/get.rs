use clap::Args;
use serde_json::{json, Value};

use crate::commands::attachments::PrincipalType;
use crate::commands::Ctx;
use crate::output;

/// `pc attachments get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Attachment id
    #[arg(value_name = "ATTACHMENT_ID")]
    pub attachment_id: String,

    /// Type of the principal the attachment belongs to
    #[arg(long, value_enum, value_name = "TYPE")]
    pub principal_type: PrincipalType,

    /// Id of the principal (work item, test run, idea, ticket, page, ...)
    #[arg(long, value_name = "ID")]
    pub principal_id: String,

    /// Comment id; pass when the attachment belongs to a comment
    #[arg(long, value_name = "ID")]
    pub comment_id: Option<String>,
}

/// 获取一个附件：`GET /v1/attachments/{attachment_id}`（scope 依赖附件所属主体）。
///
/// 查询参数 `principal_type`（附件主体类型）、`principal_id`（主体 id）必填，
/// 获取评论附件时还需传 `comment_id`。文件类型附件返回 `download_url`，
/// 代码段附件返回 `format`/`content`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getAttachmentsByAttachmentId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("principal_type".into(), json!(args.principal_type.as_str()));
    query.insert("principal_id".into(), json!(&args.principal_id));
    if let Some(comment_id) = &args.comment_id {
        query.insert("comment_id".into(), json!(comment_id));
    }

    let path = format!("/v1/attachments/{}", args.attachment_id);
    let response: Value = ctx
        .client
        .get_with_query(&path, &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
