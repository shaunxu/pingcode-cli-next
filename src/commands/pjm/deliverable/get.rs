use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm deliverable get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Deliverable target id
    #[arg(value_name = "DELIVERABLE_TARGET_ID")]
    pub deliverable_target_id: String,
}

/// 获取一个工作项交付目标：`GET /v1/pjm/deliverables/{deliverable_target_id}`
/// （scope: `pcp:read:pjm:project`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmDeliverablesByDeliverableTargetId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/deliverables/{}", args.deliverable_target_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
