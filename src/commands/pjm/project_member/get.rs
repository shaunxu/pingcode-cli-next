use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-member get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs;

/// 获取项目中的一个成员：`GET /v1/pjm/projects/{project_id}/members/{member_id}`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdMembersByMemberId
pub async fn run(_ctx: &Ctx, _args: &GetArgs) -> anyhow::Result<()> {
    todo!("GET /v1/pjm/projects/<project_id>/members/<member_id> — docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdMembersByMemberId")
}
