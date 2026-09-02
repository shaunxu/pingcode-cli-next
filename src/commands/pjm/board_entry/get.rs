use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm board-entry get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
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

/// 获取一个看板栏：`GET /v1/pjm/projects/{project_id}/boards/{board_id}/entries/{entry_id}`
/// （scope: `pcp:read:pjm:board`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdBoardsByBoardIdEntriesByEntryId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/boards/{}/entries/{}",
        args.project_id, args.board_id, args.entry_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
