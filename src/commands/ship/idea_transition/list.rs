use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship idea-transition list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Idea id
    #[arg(value_name = "IDEA_ID")]
    pub idea_id: String,
}

/// 获取需求流转记录列表：
/// `GET /v1/ship/ideas/{idea_id}/transition_histories`
/// （分页，scope: `pcp:read:ship:idea`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeasByIdeaIdTransitionHistories
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let path = format!("/v1/ship/ideas/{}/transition_histories", args.idea_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
