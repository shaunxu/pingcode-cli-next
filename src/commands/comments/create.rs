use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc comments create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    ///
    /// Required fields: principal_type (workitem|workitem_review|testrun|
    /// testcase|testcase_review|idea|idea_review|ticket|page), principal_id,
    /// content. Optional: reply_comment_id, created_at, created_by.
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一条评论：`POST /v1/comments`（scope 依赖评论所属主体，
/// 如 workitem 需要 `pcp:write:pjm:workitem`）。
///
/// 请求体必填 `principal_type`（评论主体类型：`workitem`/`workitem_review`/
/// `testrun`/`testcase`/`testcase_review`/`idea`/`idea_review`/`ticket`/`page`）、
/// `principal_id`（评论主体 id）、`content`（评论内容）；可选
/// `reply_comment_id`（被回复评论的 id）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postComments
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/comments", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
