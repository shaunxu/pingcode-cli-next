use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-state-plan add-state` 的参数。
#[derive(Debug, Args)]
pub struct AddStateArgs {
    /// Work item state plan id
    #[arg(value_name = "STATE_PLAN_ID")]
    pub state_plan_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向工作项状态方案中添加一个状态：
/// `POST /v1/pjm/workitem_state_plans/{state_plan_id}/workitem_states`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 请求体必填 `workitem_state_id`（工作项状态的 id）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemStatePlansByStatePlanIdWorkitemStates
pub async fn run(ctx: &Ctx, args: &AddStateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/pjm/workitem_state_plans/{}/workitem_states",
        args.state_plan_id
    );
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
