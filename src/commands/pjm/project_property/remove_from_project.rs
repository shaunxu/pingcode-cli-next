use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project-property remove-from-project` 的参数。
#[derive(Debug, Args)]
pub struct RemoveFromProjectArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Project property id
    #[arg(value_name = "PROPERTY_ID")]
    pub property_id: String,
}

/// 在项目中移除一个项目属性：
/// `DELETE /v1/pjm/projects/{project_id}/project_properties/{property_id}`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 返回被移除的项目内属性对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdProjectPropertiesByPropertyId
pub async fn run(ctx: &Ctx, args: &RemoveFromProjectArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/project_properties/{}",
        args.project_id, args.property_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
