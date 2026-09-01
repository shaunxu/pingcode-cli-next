use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-member update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs;

/// 部分更新项目中的一个成员：`PATCH /v1/pjm/projects/{project_id}/members/{member_id}`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdMembersByMemberId
pub async fn run(_ctx: &Ctx, _args: &UpdateArgs) -> anyhow::Result<()> {
    todo!("PATCH /v1/pjm/projects/<project_id>/members/<member_id> — docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdMembersByMemberId")
}
