use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project-property list-for-project` 的参数。
#[derive(Debug, Args)]
pub struct ListForProjectArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,
}

/// 分页获取项目中的项目属性列表：
/// `GET /v1/pjm/projects/{project_id}/project_properties`
/// （scope: `pcp:read:pjm:configuration`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdProjectProperties
pub async fn run(ctx: &Ctx, args: &ListForProjectArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/projects/{}/project_properties", args.project_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
