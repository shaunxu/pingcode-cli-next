use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project-process get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Project process id
    #[arg(value_name = "PROCESS_ID")]
    pub process_id: String,
}

/// 获取一个项目流程：`GET /v1/pjm/processes/{process_id}`
/// （scope: `pcp:read:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProcessesByProcessId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/processes/{}", args.process_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
