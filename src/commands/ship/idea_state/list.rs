use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc ship idea-state list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 分页获取全部需求状态：`GET /v1/ship/idea_states`（scope: `pcp:read:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaStates
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(i) = args.page_index {
        query.insert("page_index".into(), json!(i));
    }
    if let Some(s) = args.page_size {
        query.insert("page_size".into(), json!(s));
    }
    let response: Value = ctx
        .client
        .get_with_query("/v1/ship/idea_states", &Value::Object(query))
        .await?;
    if ctx.config.dry_run {
        return Ok(());
    }
    output::print_json(&response)?;
    Ok(())
}
