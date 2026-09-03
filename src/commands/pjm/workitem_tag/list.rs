use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-tag list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter by tag name
    #[arg(long)]
    pub name: Option<String>,
}

/// 获取全部工作项标签列表：`GET /v1/pjm/workitem_tags`（分页，
/// scope: `pcp:read:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTags
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(name) = &args.name {
        query.insert("name".into(), json!(name));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/pjm/workitem_tags", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
