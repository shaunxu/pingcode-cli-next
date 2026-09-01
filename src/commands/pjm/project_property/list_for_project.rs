use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-property list-for-project` 的参数。
#[derive(Debug, Args)]
pub struct ListForProjectArgs;

/// 获取项目中的项目属性列表：`GET /v1/pjm/projects/{project_id}/project_properties`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdProjectProperties
pub async fn run(_ctx: &Ctx, _args: &ListForProjectArgs) -> anyhow::Result<()> {
    todo!("GET /v1/pjm/projects/<project_id>/project_properties — docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdProjectProperties")
}
