use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-property add-to-project` 的参数。
#[derive(Debug, Args)]
pub struct AddToProjectArgs;

/// 向项目中添加一个项目属性：`POST /v1/pjm/projects/{project_id}/project_properties`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdProjectProperties
pub async fn run(_ctx: &Ctx, _args: &AddToProjectArgs) -> anyhow::Result<()> {
    todo!("POST /v1/pjm/projects/<project_id>/project_properties — docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdProjectProperties")
}
