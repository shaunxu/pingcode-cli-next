use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project-property update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs;

/// 部分更新一个项目属性（全局属性定义）：`PATCH /v1/pjm/project_properties/{property_id}`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectPropertiesByPropertyId
pub async fn run(_ctx: &Ctx, _args: &UpdateArgs) -> anyhow::Result<()> {
    todo!("PATCH /v1/pjm/project_properties/<property_id> — docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectPropertiesByPropertyId")
}
