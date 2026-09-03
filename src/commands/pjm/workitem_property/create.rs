use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-property create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个工作项属性：`POST /v1/pjm/workitem_properties`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 属于「工作项配置」：创建企业级工作项自定义属性字典项，完整请求字段见
/// 文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemProperties
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx
        .client
        .post("/v1/pjm/workitem_properties", &body)
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
