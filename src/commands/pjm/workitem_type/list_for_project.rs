use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-type list-for-project` 的参数。
#[derive(Debug, Args)]
pub struct ListForProjectArgs {
    /// Project id
    #[arg(long, value_name = "ID")]
    pub project_id: String,
}

/// 获取项目下的工作项类型列表：`GET /v1/pjm/workitem/types`（分页，
/// scope: `pcp:read:pjm:workitem`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTypesByProjectId
pub async fn run(ctx: &Ctx, args: &ListForProjectArgs) -> anyhow::Result<()> {
    let query = serde_json::Map::from_iter([("project_id".into(), json!(args.project_id))]);

    let response: Value = ctx
        .client
        .get_with_query("/v1/pjm/workitem/types", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
