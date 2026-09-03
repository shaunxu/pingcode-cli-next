use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Test case id or short id
    #[arg(value_name = "TESTCASE_ID")]
    pub testcase_id: String,
    /// Fields whose rich-text image tokens should be included, comma-separated
    #[arg(long, value_name = "FIELDS")]
    pub include_public_image_token: Option<String>,
    /// Include deleted test cases
    #[arg(long)]
    pub include_deleted: bool,
}

/// 获取一个测试用例（id 或 short_id）：`GET /v1/testhub/testcases/{testcase_id}`（scope: `pcp:read:testhub:testcase`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasesByTestcaseId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(include_public_image_token) = &args.include_public_image_token {
        query.insert(
            "include_public_image_token".into(),
            json!(include_public_image_token),
        );
    }
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }

    let path = format!("/v1/testhub/testcases/{}", args.testcase_id);
    let response: Value = ctx
        .client
        .get_with_query(path.as_str(), &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
