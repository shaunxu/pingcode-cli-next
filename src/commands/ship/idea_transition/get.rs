use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship idea-transition get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Idea id
    #[arg(value_name = "IDEA_ID")]
    pub idea_id: String,

    /// Transition history id
    #[arg(value_name = "TRANSITION_HISTORY_ID")]
    pub transition_history_id: String,
}

/// 获取一条需求流转记录：
/// `GET /v1/ship/ideas/{idea_id}/transition_histories/{transition_history_id}`
/// （scope: `pcp:read:ship:idea`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeasByIdeaIdTransitionHistoriesByTransitionHistoryId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/ideas/{}/transition_histories/{}",
        args.idea_id, args.transition_history_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
