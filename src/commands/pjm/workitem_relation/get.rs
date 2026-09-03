use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-relation get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Work item id
    #[arg(value_name = "WORKITEM_ID")]
    pub workitem_id: String,

    /// Relation id
    #[arg(value_name = "RELATION_ID")]
    pub relation_id: String,

    /// Include deleted relations
    #[arg(long)]
    pub include_deleted: bool,
}

/// 获取一个工作项关联：`GET /v1/pjm/workitems/{workitem_id}/relations/{relation_id}`
/// （scope: `pcp:read:pjm:workitem`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemsByWorkitemIdRelationsByRelationId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }

    let path = format!(
        "/v1/pjm/workitems/{}/relations/{}",
        args.workitem_id, args.relation_id
    );
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
