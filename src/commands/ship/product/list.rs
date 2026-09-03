use anyhow::{bail, Result};
use clap::{Args, ValueEnum};
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// 产品所属范围（查询参数 `scope_type`）。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum ScopeType {
    /// Products visible to the whole organization
    Organization,
    /// Products visible to a team (user group)
    UserGroup,
}

impl ScopeType {
    fn as_str(self) -> &'static str {
        match self {
            ScopeType::Organization => "organization",
            ScopeType::UserGroup => "user_group",
        }
    }
}

/// 产品成员类型（查询参数 `member_type`）。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum MemberType {
    /// Enterprise user
    User,
    /// Team (user group)
    UserGroup,
}

impl MemberType {
    fn as_str(self) -> &'static str {
        match self {
            MemberType::User => "user",
            MemberType::UserGroup => "user_group",
        }
    }
}

/// `pc ship product list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter by the scope the product belongs to
    #[arg(long, value_enum)]
    pub scope_type: Option<ScopeType>,

    /// Scope id; only a team (user group) id is supported
    #[arg(long, value_name = "ID")]
    pub scope_id: Option<String>,

    /// Filter by product name keyword
    #[arg(long)]
    pub keywords: Option<String>,

    /// Filter by product member type; must be used together with --member-id
    #[arg(long, value_enum, value_name = "TYPE")]
    pub member_type: Option<MemberType>,

    /// Filter by product member id (user id or team id); must be used together with --member-type
    #[arg(long, value_name = "ID")]
    pub member_id: Option<String>,

    /// Filter by creation time range, comma-separated "<start>,<end>"
    #[arg(long, value_name = "RANGE")]
    pub created_between: Option<String>,

    /// Filter by last-update time range, comma-separated "<start>,<end>"
    #[arg(long, value_name = "RANGE")]
    pub updated_between: Option<String>,

    /// Include deleted products
    #[arg(long)]
    pub include_deleted: bool,

    /// Include archived products
    #[arg(long)]
    pub include_archived: bool,

    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 分页获取产品列表：`GET /v1/ship/products`（scope: `pcp:read:ship:product`）。
///
/// 默认不含已删除/已归档产品。响应为分页结构
/// （`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipProducts
pub async fn run(ctx: &Ctx, args: &ListArgs) -> Result<()> {
    if args.member_type.is_some() != args.member_id.is_some() {
        bail!("--member-type and --member-id must be provided together");
    }

    let mut query = serde_json::Map::new();
    if let Some(scope_type) = args.scope_type {
        query.insert("scope_type".into(), json!(scope_type.as_str()));
    }
    if let Some(scope_id) = &args.scope_id {
        query.insert("scope_id".into(), json!(scope_id));
    }
    if let Some(keywords) = &args.keywords {
        query.insert("keywords".into(), json!(keywords));
    }
    if let Some(member_type) = args.member_type {
        query.insert("member_type".into(), json!(member_type.as_str()));
    }
    if let Some(member_id) = &args.member_id {
        query.insert("member_id".into(), json!(member_id));
    }
    if let Some(range) = &args.created_between {
        query.insert("created_between".into(), json!(range));
    }
    if let Some(range) = &args.updated_between {
        query.insert("updated_between".into(), json!(range));
    }
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }
    if args.include_archived {
        query.insert("include_archived".into(), json!(true));
    }
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), json!(page_size));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/ship/products", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
