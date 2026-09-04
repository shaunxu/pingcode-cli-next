use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc workload get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Workload id
    #[arg(value_name = "WORKLOAD_ID")]
    pub workload_id: String,
}

/// 获取一个工时：`GET /v1/workloads/{workload_id}`（scope: `pcp:read:global:workload`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getWorkloadsByWorkloadId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/workloads/{}", args.workload_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
