use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs;

/// 创建一个项目：`POST /v1/pjm/projects`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjects
pub async fn run(_ctx: &Ctx, _args: &CreateArgs) -> anyhow::Result<()> {
    todo!("POST /v1/pjm/projects — docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjects")
}
