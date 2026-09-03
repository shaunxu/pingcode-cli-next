use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-property list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 分页获取全部工单属性：`GET /v1/ship/ticket_properties`（scope: `pcp:read:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketProperties
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), serde_json::json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), serde_json::json!(page_size));
    }
    let response: Value = ctx
        .client
        .get_with_query("/v1/ship/ticket_properties", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
