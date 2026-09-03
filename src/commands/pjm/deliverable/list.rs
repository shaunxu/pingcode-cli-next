use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc pjm deliverable list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Project id; the project type must be waterfall or hybrid
    #[arg(long, value_name = "ID")]
    pub project_id: Option<String>,

    /// Filter by work item id
    #[arg(long, value_name = "ID")]
    pub workitem_id: Option<String>,
}

/// 获取工作项交付目标列表：`GET /v1/pjm/deliverables`（分页，
/// scope: `pcp:read:pjm:project`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmDeliverables
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(project_id) = &args.project_id {
        query.insert("project_id".into(), json!(project_id));
    }
    if let Some(workitem_id) = &args.workitem_id {
        query.insert("workitem_id".into(), json!(workitem_id));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/pjm/deliverables", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
