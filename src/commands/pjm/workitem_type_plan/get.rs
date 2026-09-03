use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-type-plan get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Work item type plan id
    #[arg(value_name = "TYPE_PLAN_ID")]
    pub workitem_type_plan_id: String,
}

/// 获取一个工作项类型方案：
/// `GET /v1/pjm/workitem_type_plans/{workitem_type_plan_id}`
/// （scope: `pcp:read:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTypePlansByWorkitemTypePlanId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/workitem_type_plans/{}", args.workitem_type_plan_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
