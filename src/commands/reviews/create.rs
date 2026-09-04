use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc reviews create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    ///
    /// Required fields: title, pilot_id, principal_type. Optional: description.
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个评审：`POST /v1/reviews`（scope 依赖评审所属主体，如工作项评审
/// 需要项目级的 `pcp:write:pjm:project`）。
///
/// 请求体必填 `title`（评审标题）、`pilot_id`（评审主体所在产品/项目/测试库
/// 的 id）、`principal_type`（评审主体类型：`workitem`/`testcase`/`idea`）；
/// 可选 `description`（评审说明）。创建后通过 `pc reviews add-principal`
/// 向评审中添加被评内容。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postReviews
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/reviews", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
