use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-state list-all` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs;

/// 获取全部工作项状态列表：`GET /v1/pjm/workitem_states`（分页，
/// scope: `pcp:read:pjm:configuration`）。
///
/// 属于「工作项配置」：分页获取企业内全部工作项状态字典项。查询某个项目
/// 下某工作项类型可用的状态用 `list-for-project`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemStates
pub async fn run(ctx: &Ctx, _args: &ListArgs) -> anyhow::Result<()> {
    let response: Value = ctx.client.get("/v1/pjm/workitem_states").await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
