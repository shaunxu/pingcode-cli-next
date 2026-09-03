use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-property list-for-project` 的参数。
#[derive(Debug, Args)]
pub struct ListForProjectArgs {
    /// Project id
    #[arg(long, value_name = "ID")]
    pub project_id: String,

    /// Work item type id
    #[arg(long, value_name = "ID")]
    pub workitem_type_id: String,
}

/// 获取项目下某工作项类型的属性列表：`GET /v1/pjm/workitem/properties`
/// （分页，scope: `pcp:read:pjm:workitem`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPropertiesByProjectIdAndWorkitemTypeId
pub async fn run(ctx: &Ctx, args: &ListForProjectArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("project_id".into(), json!(args.project_id));
    query.insert("workitem_type_id".into(), json!(args.workitem_type_id));

    let response: Value = ctx
        .client
        .get_with_query("/v1/pjm/workitem/properties", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
