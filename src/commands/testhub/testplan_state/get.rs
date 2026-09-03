use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testplan-state get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Test plan state id
    #[arg(value_name = "STATE_ID")]
    pub state_id: String,
}

/// 获取一个测试计划状态：`GET /v1/testhub/testplan_states/{state_id}`（scope: `pcp:read:testhub:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestplanStatesByStateId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/testhub/testplan_states/{}", args.state_id);
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
