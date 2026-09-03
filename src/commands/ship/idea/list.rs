use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc ship idea list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter by product id
    #[arg(long, value_name = "ID")]
    pub product_id: Option<String>,

    /// Filter by idea state id
    #[arg(long, value_name = "ID")]
    pub state_id: Option<String>,

    /// Filter by idea priority id
    #[arg(long, value_name = "ID")]
    pub priority_id: Option<String>,

    /// Filter by keyword; matches idea identifier and title
    #[arg(long)]
    pub keywords: Option<String>,

    /// Fields whose rich-text image tokens should be included, comma-separated (e.g. "description,properties.prop_b")
    #[arg(long, value_name = "FIELDS")]
    pub include_public_image_token: Option<String>,

    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 分页获取需求列表：`GET /v1/ship/ideas`（scope: `pcp:read:ship:idea`）。
///
/// 复杂组合、日期或自定义属性过滤请使用「搜索需求列表」
/// （`POST /v1/ship/ideas/search`），本命令仅支持文档列出的简单过滤参数。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeas
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(product_id) = &args.product_id {
        query.insert("product_id".into(), json!(product_id));
    }
    if let Some(state_id) = &args.state_id {
        query.insert("state_id".into(), json!(state_id));
    }
    if let Some(priority_id) = &args.priority_id {
        query.insert("priority_id".into(), json!(priority_id));
    }
    if let Some(keywords) = &args.keywords {
        query.insert("keywords".into(), json!(keywords));
    }
    if let Some(fields) = &args.include_public_image_token {
        query.insert("include_public_image_token".into(), json!(fields));
    }
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), json!(page_size));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/ship/ideas", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
