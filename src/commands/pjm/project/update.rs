use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs;

/// 部分更新一个项目：`PATCH /v1/pjm/projects/{project_id}`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectId
pub async fn run(_ctx: &Ctx, _args: &UpdateArgs) -> anyhow::Result<()> {
    todo!("PATCH /v1/pjm/projects/<project_id> — docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectId")
}
