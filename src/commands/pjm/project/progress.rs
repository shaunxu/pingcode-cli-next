use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project progress` 的参数。
#[derive(Debug, Args)]
pub struct ProgressArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,
}

/// 获取一个项目的工作项进度统计：`GET /v1/pjm/projects/{project_id}/progress`
/// （scope: `pcp:read:pjm:project`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdProgress
pub async fn run(ctx: &Ctx, args: &ProgressArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/projects/{}/progress", args.project_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
