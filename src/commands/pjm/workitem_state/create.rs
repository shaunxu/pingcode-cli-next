use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-state create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个工作项状态：`POST /v1/pjm/workitem_states`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 属于「工作项配置」：创建企业级工作项状态字典项，完整请求字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemStates
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/pjm/workitem_states", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
