use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project-property add-to-project` 的参数。
#[derive(Debug, Args)]
pub struct AddToProjectArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向项目中添加一个项目属性：
/// `POST /v1/pjm/projects/{project_id}/project_properties`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 请求体必填 `property_id`（全局项目属性的 id）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdProjectProperties
pub async fn run(ctx: &Ctx, args: &AddToProjectArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/pjm/projects/{}/project_properties", args.project_id);
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
