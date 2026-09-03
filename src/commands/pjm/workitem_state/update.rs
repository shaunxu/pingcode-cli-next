use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-state update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Work item state id
    #[arg(value_name = "STATE_ID")]
    pub state_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个工作项状态：`PATCH /v1/pjm/workitem_states/{state_id}`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 属于「工作项配置」，完整请求字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmWorkitemStatesByStateId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/pjm/workitem_states/{}", args.state_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
