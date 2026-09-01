use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-member list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs;

/// 获取项目中的成员列表：`GET /v1/pjm/projects/{project_id}/members`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdMembers
pub async fn run(_ctx: &Ctx, _args: &ListArgs) -> anyhow::Result<()> {
    todo!("GET /v1/pjm/projects/<project_id>/members — docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdMembers")
}
