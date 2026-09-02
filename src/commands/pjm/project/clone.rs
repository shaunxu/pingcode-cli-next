use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project clone` 的参数。
#[derive(Debug, Args)]
pub struct CloneArgs {
    /// Source project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 复制（克隆）一个项目：`POST /v1/pjm/projects/{project_id}/clone`
/// （scope: `pcp:write:pjm:project`）。
///
/// 请求体必填 `identifier`，可选 `scope_type`、`scope_id`、`name`、
/// `visibility`、`description`、`members`；未提供的字段沿用源项目。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdClone
pub async fn run(ctx: &Ctx, args: &CloneArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/pjm/projects/{}/clone", args.project_id);
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
