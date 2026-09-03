use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-transition list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Work item id
    #[arg(value_name = "WORKITEM_ID")]
    pub workitem_id: String,
}

/// 获取工作项流转记录列表：
/// `GET /v1/pjm/workitems/{workitem_id}/transition_histories`
/// （分页，scope: `pcp:read:pjm:workitem`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemsByWorkitemIdTransitionHistories
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/workitems/{}/transition_histories",
        args.workitem_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
