use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm deliverable update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Deliverable target id
    #[arg(value_name = "DELIVERABLE_TARGET_ID")]
    pub deliverable_target_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个工作项交付目标：
/// `PATCH /v1/pjm/deliverables/{deliverable_target_id}`
/// （scope: `pcp:write:pjm:project`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmDeliverablesByDeliverableTargetId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/pjm/deliverables/{}", args.deliverable_target_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
