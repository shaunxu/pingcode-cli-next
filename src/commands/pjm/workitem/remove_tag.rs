use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem remove-tag` 的参数。
#[derive(Debug, Args)]
pub struct RemoveTagArgs {
    /// Work item id
    #[arg(value_name = "WORKITEM_ID")]
    pub workitem_id: String,

    /// Tag id
    #[arg(value_name = "TAG_ID")]
    pub tag_id: String,
}

/// 在工作项中移除一个标签：`DELETE /v1/pjm/workitems/{workitem_id}/tags/{tag_id}`
/// （scope: `pcp:write:pjm:workitem`）。
///
/// 仅解除工作项与标签的关联，不删除标签本身；返回被移除的关联对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemsByWorkitemIdTagsByTagId
pub async fn run(ctx: &Ctx, args: &RemoveTagArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/workitems/{}/tags/{}",
        args.workitem_id, args.tag_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
