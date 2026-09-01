use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-member remove` 的参数。
#[derive(Debug, Args)]
pub struct RemoveArgs;

/// 在项目中移除一个成员：`DELETE /v1/pjm/projects/{project_id}/members/{member_id}`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdMembersByMemberId
pub async fn run(_ctx: &Ctx, _args: &RemoveArgs) -> anyhow::Result<()> {
    todo!("DELETE /v1/pjm/projects/<project_id>/members/<member_id> — docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdMembersByMemberId")
}
