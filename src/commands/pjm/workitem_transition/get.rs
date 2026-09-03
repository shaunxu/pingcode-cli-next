use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-transition get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Work item id
    #[arg(value_name = "WORKITEM_ID")]
    pub workitem_id: String,

    /// Transition history id
    #[arg(value_name = "TRANSITION_HISTORY_ID")]
    pub transition_history_id: String,
}

/// 获取一条工作项流转记录：
/// `GET /v1/pjm/workitems/{workitem_id}/transition_histories/{transition_history_id}`
/// （scope: `pcp:read:pjm:workitem`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemsByWorkitemIdTransitionHistoriesByTransitionHistoryId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/workitems/{}/transition_histories/{}",
        args.workitem_id, args.transition_history_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
