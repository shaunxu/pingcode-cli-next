use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm sprint update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Sprint id
    #[arg(value_name = "SPRINT_ID")]
    pub sprint_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个迭代：`PATCH /v1/pjm/projects/{project_id}/sprints/{sprint_id}`
/// （scope: `pcp:write:pjm:sprint`）。
///
/// 请求体可选 `name`、`start_at`、`end_at`（epoch 毫秒）、`assignee_id`、
/// `description`、`status`（pending/in_progress/completed）、`category_ids`，
/// 完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdSprintsBySprintId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/pjm/projects/{}/sprints/{}",
        args.project_id, args.sprint_id
    );
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
