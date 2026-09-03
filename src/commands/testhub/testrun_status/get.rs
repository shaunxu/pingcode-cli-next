use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testrun-status get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Test run status id
    #[arg(value_name = "STATUS_ID")]
    pub status_id: String,
}

/// 获取一个执行结果状态：`GET /v1/testhub/testrun_statuses/{status_id}`（scope: `pcp:read:testhub:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestrunStatusesByStatusId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/testhub/testrun_statuses/{}", args.status_id);
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
