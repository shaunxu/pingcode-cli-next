use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter by library id
    #[arg(long, value_name = "ID")]
    pub library_id: Option<String>,
    /// Filter by maintainer id
    #[arg(long, value_name = "ID")]
    pub maintenance_id: Option<String>,
    /// Filter by case state id
    #[arg(long, value_name = "ID")]
    pub state_id: Option<String>,
    /// Filter by important level id
    #[arg(long, value_name = "ID")]
    pub important_level_id: Option<String>,
    /// Filter by tag id
    #[arg(long, value_name = "ID")]
    pub tag_id: Option<String>,
    /// Filter by case identifier or title
    #[arg(long, value_name = "KEYWORDS")]
    pub keywords: Option<String>,
    /// Fields whose rich-text image tokens should be included, comma-separated (e.g. description,properties.prop_b)
    #[arg(long, value_name = "FIELDS")]
    pub include_public_image_token: Option<String>,
    /// Include deleted test cases
    #[arg(long)]
    pub include_deleted: bool,
}

/// 分页获取测试用例列表：`GET /v1/testhub/testcases`（scope: `pcp:read:testhub:testcase`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcases
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(library_id) = &args.library_id {
        query.insert("library_id".into(), json!(library_id));
    }
    if let Some(maintenance_id) = &args.maintenance_id {
        query.insert("maintenance_id".into(), json!(maintenance_id));
    }
    if let Some(state_id) = &args.state_id {
        query.insert("state_id".into(), json!(state_id));
    }
    if let Some(important_level_id) = &args.important_level_id {
        query.insert("important_level_id".into(), json!(important_level_id));
    }
    if let Some(tag_id) = &args.tag_id {
        query.insert("tag_id".into(), json!(tag_id));
    }
    if let Some(keywords) = &args.keywords {
        query.insert("keywords".into(), json!(keywords));
    }
    if let Some(include_public_image_token) = &args.include_public_image_token {
        query.insert(
            "include_public_image_token".into(),
            json!(include_public_image_token),
        );
    }
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/testhub/testcases", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
