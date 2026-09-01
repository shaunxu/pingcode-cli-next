use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-property remove-from-project` 的参数。
#[derive(Debug, Args)]
pub struct RemoveFromProjectArgs;

/// 在项目中移除一个项目属性：`DELETE /v1/pjm/projects/{project_id}/project_properties/{property_id}`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdProjectPropertiesByPropertyId
pub async fn run(_ctx: &Ctx, _args: &RemoveFromProjectArgs) -> anyhow::Result<()> {
    todo!("DELETE /v1/pjm/projects/<project_id>/project_properties/<property_id> — docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdProjectPropertiesByPropertyId")
}
