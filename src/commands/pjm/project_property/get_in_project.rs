use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project-property get-in-project` 的参数。
#[derive(Debug, Args)]
pub struct GetInProjectArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Project property id
    #[arg(value_name = "PROPERTY_ID")]
    pub property_id: String,
}

/// 获取项目中的一个项目属性：
/// `GET /v1/pjm/projects/{project_id}/project_properties/{property_id}`
/// （scope: `pcp:read:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdProjectPropertiesByPropertyId
pub async fn run(ctx: &Ctx, args: &GetInProjectArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/project_properties/{}",
        args.project_id, args.property_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
