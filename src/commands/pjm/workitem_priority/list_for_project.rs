use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-priority list-for-project` 的参数。
#[derive(Debug, Args)]
pub struct ListForProjectArgs {
    /// Project id
    #[arg(long, value_name = "ID")]
    pub project_id: String,
}

/// 获取项目下的工作项优先级列表：`GET /v1/pjm/workitem/priorities`
/// （分页，scope: `pcp:read:pjm:workitem`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPrioritiesByProjectId
pub async fn run(ctx: &Ctx, args: &ListForProjectArgs) -> anyhow::Result<()> {
    let query = serde_json::Map::from_iter([("project_id".into(), json!(args.project_id))]);

    let response: Value = ctx
        .client
        .get_with_query("/v1/pjm/workitem/priorities", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
