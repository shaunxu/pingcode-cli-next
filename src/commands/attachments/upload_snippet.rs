use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc attachments upload-snippet` 的参数。
#[derive(Debug, Args)]
pub struct UploadSnippetArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    ///
    /// Required fields: principal_type, principal_id, title, format, content.
    /// Optional: comment_id. Note that workitem_deliverable is not supported
    /// for snippet uploads.
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 上传一个代码段附件：`POST /v1/attachments`（`Content-Type: application/json`，
/// scope 依赖附件所属主体）。
///
/// 请求体必填 `principal_type`（附件主体类型，代码段不支持
/// `workitem_deliverable`）、`principal_id`（主体 id）、`title`（标题）、
/// `format`（代码语言，如 `rust`/`python`/`javascript`/`shell`/`sql`/
/// `markdown` 等）、`content`（代码内容）；挂到评论上时传 `comment_id`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postAttachments
pub async fn run(ctx: &Ctx, args: &UploadSnippetArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/attachments", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
