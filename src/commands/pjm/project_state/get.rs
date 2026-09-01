use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-state get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs;

/// 获取一个项目状态：`GET /v1/pjm/project_states/{state_id}`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectStatesByStateId
pub async fn run(_ctx: &Ctx, _args: &GetArgs) -> anyhow::Result<()> {
    todo!("GET /v1/pjm/project_states/<state_id> — docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectStatesByStateId")
}
