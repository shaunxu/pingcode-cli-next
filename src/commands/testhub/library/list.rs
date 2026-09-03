use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc testhub library list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter by scope type: organization or user_group
    #[arg(long, value_name = "SCOPE")]
    pub scope_type: Option<String>,
    /// Filter by scope (team) id; used with scope-type=user_group
    #[arg(long, value_name = "ID")]
    pub scope_id: Option<String>,
    /// Filter by library name
    #[arg(long, value_name = "KEYWORDS")]
    pub keywords: Option<String>,
    /// Filter by member type: user or user_group (requires --member-id)
    #[arg(long, value_name = "TYPE")]
    pub member_type: Option<String>,
    /// Filter by member id (requires --member-type)
    #[arg(long, value_name = "ID")]
    pub member_id: Option<String>,
    /// Filter by creation time, epoch seconds, e.g. 1700000000,1700100000
    #[arg(long, value_name = "RANGE")]
    pub created_between: Option<String>,
    /// Filter by update time, epoch seconds
    #[arg(long, value_name = "RANGE")]
    pub updated_between: Option<String>,
    /// Include deleted libraries
    #[arg(long)]
    pub include_deleted: bool,
    /// Include archived libraries
    #[arg(long)]
    pub include_archived: bool,
}

/// 分页获取测试库列表：`GET /v1/testhub/libraries`（scope: `pcp:read:testhub:library`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibraries
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(scope_type) = &args.scope_type {
        query.insert("scope_type".into(), json!(scope_type));
    }
    if let Some(scope_id) = &args.scope_id {
        query.insert("scope_id".into(), json!(scope_id));
    }
    if let Some(keywords) = &args.keywords {
        query.insert("keywords".into(), json!(keywords));
    }
    if let Some(member_type) = &args.member_type {
        query.insert("member_type".into(), json!(member_type));
    }
    if let Some(member_id) = &args.member_id {
        query.insert("member_id".into(), json!(member_id));
    }
    if let Some(created_between) = &args.created_between {
        query.insert("created_between".into(), json!(created_between));
    }
    if let Some(updated_between) = &args.updated_between {
        query.insert("updated_between".into(), json!(updated_between));
    }
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }
    if args.include_archived {
        query.insert("include_archived".into(), json!(true));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/testhub/libraries", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
