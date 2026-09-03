use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个工作项：`POST /v1/pjm/workitems`（scope: `pcp:write:pjm:workitem`）。
///
/// 请求体必填 `project_id`（项目 id）、`type_id`（工作项类型 id 或系统类型枚举）、
/// `title`（标题）；可选 `description`、`start_at`、`end_at`、`state_id`、
/// `parent_id`、`sprint_id`、`release_ids`、`board_id`、`entry_id`、
/// `swimlane_id`、`priority_id`、`assignee_id`、`participant_ids`、
/// `story_points`、`estimated_workload`、`remaining_workload`、`properties` 等字段。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitems
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/pjm/workitems", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
