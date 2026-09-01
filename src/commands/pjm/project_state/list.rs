use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-state list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs;

/// 获取项目状态列表：`GET /v1/pjm/project/states?project_id={project_id}`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectStatesByProjectId
pub async fn run(_ctx: &Ctx, _args: &ListArgs) -> anyhow::Result<()> {
    todo!("GET /v1/pjm/project/states?project_id=<project_id> — docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectStatesByProjectId")
}
