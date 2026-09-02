use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project enable-local-config` 的参数。
#[derive(Debug, Args)]
pub struct EnableLocalConfigArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,
}

/// 开启项目本地配置：`POST /v1/pjm/projects/{project_id}/local_config/enable`
/// （scope: `pcp:write:pjm:project`）。
///
/// 该接口无请求体字段，返回更新后的项目对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdLocalConfigEnable
pub async fn run(ctx: &Ctx, args: &EnableLocalConfigArgs) -> anyhow::Result<()> {
    let body = json!({});

    let path = format!("/v1/pjm/projects/{}/local_config/enable", args.project_id);
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
