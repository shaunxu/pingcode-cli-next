use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Include deleted projects
    #[arg(long)]
    pub include_deleted: bool,

    /// Include archived projects
    #[arg(long)]
    pub include_archived: bool,
}

/// 获取一个项目：`GET /v1/pjm/projects/{project_id}`（scope: `pcp:read:pjm:project`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }
    if args.include_archived {
        query.insert("include_archived".into(), json!(true));
    }

    let path = format!("/v1/pjm/projects/{}", args.project_id);
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
