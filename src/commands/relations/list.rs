use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc relations list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Type of the relation's principal (e.g. idea, ticket, workitem)
    #[arg(long, value_name = "TYPE")]
    pub principal_type: String,

    /// Id of the relation's principal
    #[arg(long, value_name = "ID")]
    pub principal_id: String,

    /// Type of the relation target; must match the principal type pairing
    /// (e.g. principal_type=idea with target_type=ticket)
    #[arg(long, value_name = "TYPE")]
    pub target_type: String,

    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 获取关联列表：`GET /v1/relations`（分页，scope 同时依赖关联主体和关联目标
/// 的作用域，如关联需求与工单需要 `pcp:read:ship:idea` + `pcp:read:ship:ticket`）。
///
/// 查询参数 `principal_type`/`principal_id`/`target_type` 均必填，
/// 主体类型与目标类型需搭配使用（如 `idea` ↔ `ticket`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getRelationsByPrincipalTypeAndPrincipalIdAndTargetType
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("principal_type".into(), json!(&args.principal_type));
    query.insert("principal_id".into(), json!(&args.principal_id));
    query.insert("target_type".into(), json!(&args.target_type));
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), json!(page_size));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/relations", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
