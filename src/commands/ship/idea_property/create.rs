use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship idea-property create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个需求属性：`POST /v1/ship/idea_properties`（scope: `pcp:write:ship:configuration`）。
///
/// 请求体必填 `name`（企业内唯一）、`type`（text/textarea/select/multi_select/
/// cascade_select/cascade_multi_select/member/members/date/number/progress/rate/link），
/// 选择类属性可选 `options`，完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postShipIdeaProperties
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;
    let response: Value = ctx.client.post("/v1/ship/idea_properties", &body).await?;
    if ctx.config.dry_run {
        return Ok(());
    }
    output::print_json(&response)?;
    Ok(())
}
