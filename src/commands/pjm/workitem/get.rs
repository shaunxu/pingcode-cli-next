use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Work item id or short id
    #[arg(value_name = "WORKITEM_ID")]
    pub workitem_id: String,

    /// Fields whose rich-text image tokens should be included, comma-separated (e.g. "description,properties.prop_b")
    #[arg(long, value_name = "FIELDS")]
    pub include_public_image_token: Option<String>,

    /// Include deleted work items
    #[arg(long)]
    pub include_deleted: bool,

    /// Include archived work items
    #[arg(long)]
    pub include_archived: bool,
}

/// 获取一个工作项：`GET /v1/pjm/workitems/{workitem_id}`
/// （scope: `pcp:read:pjm:workitem`）。
///
/// 路径参数支持工作项的 id 或 short_id；默认不含已删除/已归档项。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemsByWorkitemId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(fields) = &args.include_public_image_token {
        query.insert("include_public_image_token".into(), json!(fields));
    }
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }
    if args.include_archived {
        query.insert("include_archived".into(), json!(true));
    }

    let path = format!("/v1/pjm/workitems/{}", args.workitem_id);
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
