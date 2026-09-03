use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship idea-state get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Idea state id
    #[arg(value_name = "IDEA_STATE_ID")]
    pub idea_state_id: String,
}

/// 获取一个需求状态：`GET /v1/ship/idea_states/{idea_state_id}`（scope: `pcp:read:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaStatesByIdeaStateId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/idea_states/{idea_state_id}",
        idea_state_id = args.idea_state_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
