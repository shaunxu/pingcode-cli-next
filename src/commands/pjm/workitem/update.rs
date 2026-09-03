use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Work item id
    #[arg(value_name = "WORKITEM_ID")]
    pub workitem_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个工作项：`PATCH /v1/pjm/workitems/{workitem_id}`
/// （scope: `pcp:write:pjm:workitem`）。
///
/// 请求体可选 `title`、`description`、`start_at`、`end_at`、`sprint_id`、
/// `release_ids`、`priority_id`、`assignee_id`、`story_points`、`state_id`、
/// `parent_id`、`properties`、`board_id`、`entry_id`、`swimlane_id`、
/// `phase_id`、`estimated_workload`、`remaining_workload` 等字段；
/// 字段生效条件（项目类型、属性方案、状态流转等）见官方文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmWorkitemsByWorkitemId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/pjm/workitems/{}", args.workitem_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
