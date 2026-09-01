use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project progress` 的参数。
#[derive(Debug, Args)]
pub struct ProgressArgs;

/// 获取一个项目进度：`GET /v1/pjm/projects/{project_id}/progress`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdProgress
pub async fn run(_ctx: &Ctx, _args: &ProgressArgs) -> anyhow::Result<()> {
    todo!("GET /v1/pjm/projects/<project_id>/progress — docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdProgress")
}
