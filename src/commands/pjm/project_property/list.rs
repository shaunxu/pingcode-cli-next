use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-property list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs;

/// 获取项目属性列表（全局属性定义）：`GET /v1/pjm/project_properties`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectProperties
pub async fn run(_ctx: &Ctx, _args: &ListArgs) -> anyhow::Result<()> {
    todo!("GET /v1/pjm/project_properties — docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectProperties")
}
