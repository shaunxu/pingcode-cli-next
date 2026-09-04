use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc workload-type get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Workload type id
    #[arg(value_name = "TYPE_ID")]
    pub type_id: String,
}

/// 获取一个工时类型：`GET /v1/workload_types/{type_id}`
/// （scope: `pcp:read:global:workload`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getWorkloadTypesByTypeId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/workload_types/{}", args.type_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
