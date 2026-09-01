use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-property get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs;

/// 获取一个项目属性（全局属性定义）：`GET /v1/pjm/project_properties/{property_id}`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectPropertiesByPropertyId
pub async fn run(_ctx: &Ctx, _args: &GetArgs) -> anyhow::Result<()> {
    todo!("GET /v1/pjm/project_properties/<property_id> — docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectPropertiesByPropertyId")
}
