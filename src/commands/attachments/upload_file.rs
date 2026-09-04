use std::path::Path;

use anyhow::Context;
use clap::Args;
use serde_json::{json, Value};

use crate::client::MultipartField;
use crate::commands::attachments::PrincipalType;
use crate::commands::Ctx;
use crate::output;

/// `pc attachments upload-file` 的参数。
#[derive(Debug, Args)]
pub struct UploadFileArgs {
    /// Type of the principal to attach the file to
    #[arg(long, value_enum, value_name = "TYPE")]
    pub principal_type: PrincipalType,

    /// Id of the principal (work item, test run, idea, ticket, page, ...)
    #[arg(long, value_name = "ID")]
    pub principal_id: String,

    /// Path to the local file to upload
    #[arg(long, value_name = "PATH")]
    pub file: String,

    /// Attachment title (file name); defaults to the file name of --file
    #[arg(long, value_name = "TITLE")]
    pub title: Option<String>,

    /// Comment id; pass to attach the file to a comment on the principal
    #[arg(long, value_name = "ID")]
    pub comment_id: Option<String>,
}

/// 上传一个文件附件：`POST /v1/attachments?principal_type=...&principal_id=...`
/// （`Content-Type: multipart/form-data`，scope 依赖附件所属主体）。
///
/// 查询参数 `principal_type`（附件主体类型）、`principal_id`（主体 id）必填，
/// 向评论上传时还需传 `comment_id`。multipart 表单字段：`title`（文件名）、
/// `file`（文件二进制内容）；`title` 缺省取 `--file` 的文件名。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postAttachmentsByPrincipalTypeAndPrincipalId
pub async fn run(ctx: &Ctx, args: &UploadFileArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("principal_type".into(), json!(args.principal_type.as_str()));
    query.insert("principal_id".into(), json!(&args.principal_id));
    if let Some(comment_id) = &args.comment_id {
        query.insert("comment_id".into(), json!(comment_id));
    }

    let path = Path::new(&args.file);
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .context("--file must point to a file with a valid file name")?;
    let title = args.title.as_deref().unwrap_or(file_name);

    // dry-run 不读本地文件，仅预览 multipart 表单摘要。
    let preview = json!({
        "multipart_form": {
            "title": title,
            "file": format!("@{}", args.file),
        }
    });
    if ctx.config.dry_run {
        let _ = ctx
            .client
            .post_multipart::<Value>("/v1/attachments", &Value::Object(query), &[], &preview)
            .await?;
        return Ok(());
    }

    let bytes = std::fs::read(path)
        .with_context(|| format!("failed to read attachment file: {}", args.file))?;

    let fields = [
        MultipartField::Text("title", title),
        MultipartField::File {
            name: "file",
            file_name,
            bytes: &bytes,
        },
    ];

    let response: Value = ctx
        .client
        .post_multipart("/v1/attachments", &Value::Object(query), &fields, &preview)
        .await?;

    output::print_json(&response)?;
    Ok(())
}
