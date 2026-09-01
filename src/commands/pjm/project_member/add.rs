use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-member add` 的参数。
#[derive(Debug, Args)]
pub struct AddArgs;

/// 向项目中添加一个成员：`POST /v1/pjm/projects/{project_id}/members`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdMembers
pub async fn run(_ctx: &Ctx, _args: &AddArgs) -> anyhow::Result<()> {
    todo!("POST /v1/pjm/projects/<project_id>/members — docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdMembers")
}
