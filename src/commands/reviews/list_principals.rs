use clap::Args;
use serde_json::{json, Value};

use crate::commands::reviews::PrincipalType;
use crate::commands::Ctx;
use crate::output;

/// `pc reviews list-principals` 的参数。
#[derive(Debug, Args)]
pub struct ListPrincipalsArgs {
    /// Review id
    #[arg(value_name = "REVIEW_ID")]
    pub review_id: String,

    /// Type of the reviewed principals (workitem, testcase or idea)
    #[arg(long, value_enum, value_name = "TYPE")]
    pub principal_type: PrincipalType,

    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 获取评审中的评审内容列表：`GET /v1/reviews/{review_id}/principals`
/// （分页，scope 依赖评审所属主体）。
///
/// 查询参数 `principal_type`（评审主体类型：`workitem`/`testcase`/`idea`）
/// 必填，`page_index`/`page_size` 为分页参数。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getReviewsByReviewIdPrincipalsByPrincipalType
pub async fn run(ctx: &Ctx, args: &ListPrincipalsArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("principal_type".into(), json!(args.principal_type.as_str()));
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), json!(page_size));
    }

    let path = format!("/v1/reviews/{}/principals", args.review_id);
    let response: Value = ctx
        .client
        .get_with_query(&path, &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
