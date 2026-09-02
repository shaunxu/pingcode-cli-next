use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project-property create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个项目属性（全局属性定义）：`POST /v1/pjm/project_properties`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 请求体必填 `name`（企业内唯一）、`type`（text / textarea / select /
/// multi_select / cascade_select / cascade_multi_select / member / members /
/// date / number / progress / rate / link）；当类型为 select /
/// multi_select / cascade_select / cascade_multi_select 时可选填 `options`
/// （下拉选项列表），完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectProperties
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/pjm/project_properties", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
