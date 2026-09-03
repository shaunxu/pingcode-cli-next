use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-state-plan add-flow` 的参数。
#[derive(Debug, Args)]
pub struct AddFlowArgs {
    /// Work item state plan id
    #[arg(value_name = "STATE_PLAN_ID")]
    pub state_plan_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向工作项状态方案中添加一条状态流转：
/// `POST /v1/pjm/workitem_state_plans/{state_plan_id}/workitem_state_flows`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 请求体必填 `from_state_id`（起始工作项状态 id）与 `to_state_id`
/// （可流转到的目标工作项状态 id）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemStatePlansByStatePlanIdWorkitemStateFlows
pub async fn run(ctx: &Ctx, args: &AddFlowArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/pjm/workitem_state_plans/{}/workitem_state_flows",
        args.state_plan_id
    );
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
