use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-property get-in-project` 的参数。
#[derive(Debug, Args)]
pub struct GetInProjectArgs;

/// 获取项目中的一个项目属性：`GET /v1/pjm/projects/{project_id}/project_properties/{property_id}`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdProjectPropertiesByPropertyId
pub async fn run(_ctx: &Ctx, _args: &GetInProjectArgs) -> anyhow::Result<()> {
    todo!("GET /v1/pjm/projects/<project_id>/project_properties/<property_id> — docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdProjectPropertiesByPropertyId")
}
