use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc wiki space get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Space id
    #[arg(value_name = "SPACE_ID")]
    pub space_id: String,

    /// Include deleted spaces
    #[arg(long)]
    pub include_deleted: bool,

    /// Include archived spaces
    #[arg(long)]
    pub include_archived: bool,
}

/// 获取一个空间：`GET /v1/wiki/spaces/{space_id}`
/// （scope: `pcp:read:wiki:space`）。
///
/// 按 id 获取空间全量信息，默认不含已删除/已归档空间。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getWikiSpacesBySpaceId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }
    if args.include_archived {
        query.insert("include_archived".into(), json!(true));
    }

    let path = format!("/v1/wiki/spaces/{}", args.space_id);
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
