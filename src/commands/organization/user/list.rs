use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc organization user list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter by member name (unique within an enterprise)
    #[arg(long)]
    pub name: Option<String>,

    /// Fuzzy keyword search over display name and user name
    #[arg(long)]
    pub keywords: Option<String>,

    /// Filter by email addresses, comma-separated (up to 20)
    #[arg(long, value_name = "EMAILS")]
    pub emails: Option<String>,

    /// Filter by mobile numbers, comma-separated (up to 20)
    #[arg(long, value_name = "MOBILES")]
    pub mobiles: Option<String>,

    /// Filter by department ids, comma-separated (up to 20)
    #[arg(long, value_name = "IDS")]
    pub department_ids: Option<String>,

    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 分页获取企业成员列表：`GET /v1/directory/users`
/// （scope: `pcp:read:global:team`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryUsers
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(name) = &args.name {
        query.insert("name".into(), json!(name));
    }
    if let Some(keywords) = &args.keywords {
        query.insert("keywords".into(), json!(keywords));
    }
    if let Some(emails) = &args.emails {
        query.insert("emails".into(), json!(emails));
    }
    if let Some(mobiles) = &args.mobiles {
        query.insert("mobiles".into(), json!(mobiles));
    }
    if let Some(department_ids) = &args.department_ids {
        query.insert("department_ids".into(), json!(department_ids));
    }
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), json!(page_size));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/directory/users", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
