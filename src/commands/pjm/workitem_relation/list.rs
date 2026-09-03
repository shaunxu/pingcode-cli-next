use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-relation list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Work item id
    #[arg(value_name = "WORKITEM_ID")]
    pub workitem_id: String,

    /// Filter by relation type (e.g. relate, block, blocked_by, or a custom type id)
    #[arg(long, value_name = "TYPE")]
    pub relation_type: Option<String>,
}

/// 获取工作项关联列表：`GET /v1/pjm/workitems/{workitem_id}/relations`
/// （分页，scope: `pcp:read:pjm:workitem`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemsByWorkitemIdRelations
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(relation_type) = &args.relation_type {
        query.insert("relation_type".into(), json!(relation_type));
    }

    let path = format!("/v1/pjm/workitems/{}/relations", args.workitem_id);
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
