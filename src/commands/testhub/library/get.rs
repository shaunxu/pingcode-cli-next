use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc testhub library get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
    /// Include deleted libraries
    #[arg(long)]
    pub include_deleted: bool,
    /// Include archived libraries
    #[arg(long)]
    pub include_archived: bool,
}

/// 获取一个测试库：`GET /v1/testhub/libraries/{library_id}`（scope: `pcp:read:testhub:library`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }
    if args.include_archived {
        query.insert("include_archived".into(), json!(true));
    }

    let path = format!("/v1/testhub/libraries/{}", args.library_id);
    let response: Value = ctx
        .client
        .get_with_query(path.as_str(), &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
