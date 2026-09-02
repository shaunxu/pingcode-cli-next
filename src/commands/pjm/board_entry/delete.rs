use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm board-entry delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Board id
    #[arg(value_name = "BOARD_ID")]
    pub board_id: String,

    /// Board entry id
    #[arg(value_name = "ENTRY_ID")]
    pub entry_id: String,
}

/// 删除一个看板栏：`DELETE /v1/pjm/projects/{project_id}/boards/{board_id}/entries/{entry_id}`
/// （scope: `pcp:write:pjm:board`）。
///
/// 返回被删除的看板栏对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdBoardsByBoardIdEntriesByEntryId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/boards/{}/entries/{}",
        args.project_id, args.board_id, args.entry_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
