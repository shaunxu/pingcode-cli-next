use clap::Args;
use serde_json::{json, Value};

use crate::commands::reviews::{PrincipalType, ReviewStatus};
use crate::commands::Ctx;
use crate::output;

/// `pc reviews list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Type of the reviewed principals (workitem, testcase or idea)
    #[arg(long, value_enum, value_name = "TYPE")]
    pub principal_type: PrincipalType,

    /// Id of the pilot the reviews belong to (project id, product id or test library id)
    #[arg(long, value_name = "ID")]
    pub pilot_id: String,

    /// Filter by review status
    #[arg(long, value_enum, value_name = "STATUS")]
    pub status: Option<ReviewStatus>,

    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 获取评审列表：`GET /v1/reviews`（分页，scope 依赖评审所属主体，如工作项
/// 评审需要项目级的 `pcp:read:pjm:project`）。
///
/// 查询参数：
/// - `principal_type`：评审主体类型（`workitem`/`testcase`/`idea`）；
/// - `pilot_id`：评审主体所在产品/项目/测试库的 id（注意是容器 id，
///   不是被评工作项/需求/用例的 id）；
/// - `status`：评审状态过滤（`pending`/`in_progress`/`completed`/
///   `repealed`）；
/// - `page_index`/`page_size`：分页参数。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getReviewsByPrincipalTypeAndPilotId
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("principal_type".into(), json!(args.principal_type.as_str()));
    query.insert("pilot_id".into(), json!(&args.pilot_id));
    if let Some(status) = args.status {
        query.insert("status".into(), json!(status.as_str()));
    }
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), json!(page_size));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/reviews", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
