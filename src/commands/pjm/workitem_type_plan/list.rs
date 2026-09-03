use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-type-plan list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Project id; when set, returns the work item type plan applied to that project
    #[arg(long, value_name = "ID")]
    pub project_id: Option<String>,
}

/// 获取工作项类型方案列表：`GET /v1/pjm/workitem_type_plans`（分页，
/// scope: `pcp:read:pjm:configuration`）。
///
/// 传入 `--project-id` 时通过查询参数 `project_id` 返回指定项目应用的
/// 工作项类型方案。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTypePlans
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let response: Value = if let Some(project_id) = &args.project_id {
        ctx.client
            .get_with_query(
                "/v1/pjm/workitem_type_plans",
                &json!({ "project_id": project_id }),
            )
            .await?
    } else {
        ctx.client.get("/v1/pjm/workitem_type_plans").await?
    };

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
