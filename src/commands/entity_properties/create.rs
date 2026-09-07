use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc entity-properties create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    ///
    /// Required fields: entity_type (workitem|idea|ticket|testcase), entity_id,
    /// property_key, value (any JSON <= 2KB), overwrite (1|0).
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向实体中添加一个扩展属性：`POST /v1/entity_properties`
/// （scope 依赖所属实体，如 workitem 需要 `pcp:write:pjm:workitem`）。
///
/// 请求体必填 `entity_type`（实体类型：`workitem`/`idea`/`ticket`/`testcase`）、
/// `entity_id`（实体 id）、`property_key`（属性键，仅允许字母数字及 `:._-`，
/// 最长 256）、`value`（任意合法 JSON，不超过 2KB）、`overwrite`（相同
/// property_key 已存在时是否覆盖：`1` 覆盖、`0` 不覆盖）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postEntityProperties
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/entity_properties", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
