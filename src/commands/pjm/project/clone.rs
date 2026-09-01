use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project clone` 的参数。
#[derive(Debug, Args)]
pub struct CloneArgs;

/// 复制一个项目：`POST /v1/pjm/projects/{project_id}/clone`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdClone
pub async fn run(_ctx: &Ctx, _args: &CloneArgs) -> anyhow::Result<()> {
    todo!("POST /v1/pjm/projects/<project_id>/clone — docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdClone")
}
