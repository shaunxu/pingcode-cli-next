use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm board-swimlane update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Board id
    #[arg(value_name = "BOARD_ID")]
    pub board_id: String,

    /// Swimlane id
    #[arg(value_name = "SWIMLANE_ID")]
    pub swimlane_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个泳道：`PATCH /v1/pjm/projects/{project_id}/boards/{board_id}/swimlanes/{swimlane_id}`
/// （scope: `pcp:write:pjm:board`）。
///
/// 请求体可选 `name`（同一看板下唯一）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdBoardsByBoardIdSwimlanesBySwimlaneId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/pjm/projects/{}/boards/{}/swimlanes/{}",
        args.project_id, args.board_id, args.swimlane_id
    );
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
