use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization job get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Job id
    #[arg(value_name = "JOB_ID")]
    pub job_id: String,
}

/// 获取一个职位：`GET /v1/directory/jobs/{job_id}`
/// （scope: `pcp:read:global:team`）。
///
/// 按 id 获取职位全量信息，含是否为系统内置（`is_system`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryJobsByJobId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/directory/jobs/{}", args.job_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
