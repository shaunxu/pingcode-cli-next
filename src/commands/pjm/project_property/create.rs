use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-property create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs;

/// 创建一个项目属性（全局属性定义）：`POST /v1/pjm/project_properties`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectProperties
pub async fn run(_ctx: &Ctx, _args: &CreateArgs) -> anyhow::Result<()> {
    todo!("POST /v1/pjm/project_properties — docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectProperties")
}
