use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc ship idea get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Idea id
    #[arg(value_name = "IDEA_ID")]
    pub idea_id: String,

    /// Fields whose rich-text image tokens should be included, comma-separated (e.g. "description,properties.prop_b")
    #[arg(long, value_name = "FIELDS")]
    pub include_public_image_token: Option<String>,

    /// Include deleted ideas
    #[arg(long)]
    pub include_deleted: bool,
}

/// 获取一个需求：`GET /v1/ship/ideas/{idea_id}`（scope: `pcp:read:ship:idea`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeasByIdeaId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(fields) = &args.include_public_image_token {
        query.insert("include_public_image_token".into(), json!(fields));
    }
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }

    let path = format!("/v1/ship/ideas/{}", args.idea_id);
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
