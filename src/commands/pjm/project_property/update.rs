use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project-property update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Project property id
    #[arg(value_name = "PROPERTY_ID")]
    pub property_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个项目属性（全局属性定义）：
/// `PATCH /v1/pjm/project_properties/{property_id}`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 请求体可选 `name`（企业内唯一）、`options`（下拉选项列表，整体更新），
/// 完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectPropertiesByPropertyId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/pjm/project_properties/{}", args.property_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
