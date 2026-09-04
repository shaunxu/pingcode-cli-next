use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc workload delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Workload id
    #[arg(value_name = "WORKLOAD_ID")]
    pub workload_id: String,
}

/// 删除一个工时：`DELETE /v1/workloads/{workload_id}`
/// （scope: `pcp:write:global:workload`）。
///
/// 用户令牌只能删除自己登记的工时，企业令牌不限。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteWorkloadsByWorkloadId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!("/v1/workloads/{}", args.workload_id);
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
