use clap::Args;

use crate::commands::Ctx;

/// `pc pjm project enable-local-config` 的参数。
#[derive(Debug, Args)]
pub struct EnableLocalConfigArgs;

/// 开启项目本地配置：`POST /v1/pjm/projects/{project_id}/local_config/enable`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdLocalConfigEnable
pub async fn run(_ctx: &Ctx, _args: &EnableLocalConfigArgs) -> anyhow::Result<()> {
    todo!("POST /v1/pjm/projects/<project_id>/local_config/enable — docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdLocalConfigEnable")
}
