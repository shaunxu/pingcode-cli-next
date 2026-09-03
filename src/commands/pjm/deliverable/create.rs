use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm deliverable create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个工作项交付目标：`POST /v1/pjm/deliverables`
/// （scope: `pcp:write:pjm:project`）。
///
/// 所属项目须为 waterfall 或 hybrid。请求体必填 `workitem_id`
/// （工作项 id，其所属项目类型必须为 waterfall 或 hybrid）与 `name`；
/// 可选 `content_type`（交付物类型，只支持 `link`；附件类型通过
/// 「上传一个文件」接口上传）与 `content`（link 类型时为
/// `{ "name": ..., "href": ... }` 对象，交付物非空时必须带 `content_type`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmDeliverables
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/pjm/deliverables", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
