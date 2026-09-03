use clap::{Args, ValueEnum};
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// 正文格式（查询参数 `format_type`）。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum FormatType {
    /// Plain text
    Text,
    /// Markdown
    Markdown,
    /// HTML
    Html,
    /// Block (structured) format
    Block,
}

impl FormatType {
    fn as_str(self) -> &'static str {
        match self {
            FormatType::Text => "text",
            FormatType::Markdown => "markdown",
            FormatType::Html => "html",
            FormatType::Block => "block",
        }
    }
}

/// `pc wiki page get-content` 的参数。
#[derive(Debug, Args)]
pub struct GetContentArgs {
    /// Page id
    #[arg(value_name = "PAGE_ID")]
    pub page_id: String,

    /// Content format (defaults to text on the server)
    #[arg(long, value_enum, value_name = "FORMAT")]
    pub format_type: Option<FormatType>,

    /// Page version id; defaults to the page's current version
    #[arg(long, value_name = "ID")]
    pub version_id: Option<String>,

    /// Include image tokens for the given rich-text field, e.g. "content"; only valid for markdown/html/block
    #[arg(long, value_name = "FIELDS")]
    pub include_public_image_token: Option<String>,
}

/// 获取一个文档正文：`GET /v1/wiki/pages/{page_id}/content`
/// （scope: `pcp:read:wiki:page`）。
///
/// `--format-type` 支持 text/markdown/html/block（服务端默认 text）；
/// `--include-public-image-token` 仅在 markdown/html/block 格式下有效。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getWikiPagesByPageIdContent
pub async fn run(ctx: &Ctx, args: &GetContentArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(format_type) = args.format_type {
        query.insert("format_type".into(), json!(format_type.as_str()));
    }
    if let Some(version_id) = &args.version_id {
        query.insert("version_id".into(), json!(version_id));
    }
    if let Some(fields) = &args.include_public_image_token {
        query.insert("include_public_image_token".into(), json!(fields));
    }

    let path = format!("/v1/wiki/pages/{}/content", args.page_id);
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
