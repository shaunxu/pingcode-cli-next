use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm board-entry update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Board id
    #[arg(value_name = "BOARD_ID")]
    pub board_id: String,

    /// Board entry id
    #[arg(value_name = "ENTRY_ID")]
    pub entry_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个看板栏：`PATCH /v1/pjm/projects/{project_id}/boards/{board_id}/entries/{entry_id}`
/// （scope: `pcp:write:pjm:board`）。
///
/// 请求体可选 `name`（同一看板下唯一）、`wip_limit`（在制品数量）、
/// `is_split`（是否拆分为进行中和已完成，默认 false）、
/// `definition_of_done`（完成的定义）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdBoardsByBoardIdEntriesByEntryId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/pjm/projects/{}/boards/{}/entries/{}",
        args.project_id, args.board_id, args.entry_id
    );
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
