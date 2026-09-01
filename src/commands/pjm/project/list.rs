use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs;

/// 获取项目列表：`GET /v1/pjm/projects`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjects
pub async fn run(_ctx: &Ctx, _args: &ListArgs) -> anyhow::Result<()> {
    todo!("GET /v1/pjm/projects — docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjects")
}
