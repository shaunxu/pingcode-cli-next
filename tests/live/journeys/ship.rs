//! 旅程 I：ship 产品（product）+ 需求（idea）+ 工单（ticket）+ 需求模块/标签。
//!
//! 注意：产品、需求、工单都没有删除接口，创建后在租户保留（pcl 前缀便于人工清理）。
//! 需求模块（suite）和标签（tag）有删除接口，会清理；产品成员 add/remove 不留残留。

use serde_json::json;

use crate::common::{find_by_id, str_field, values, LiveCtx};
use crate::journeys::support;

#[test]
fn journey_ship_product_idea_ticket() {
    let Some(ctx) = LiveCtx::new() else {
        return;
    };

    let mut state = State::default();
    let result = run_steps(&ctx, &mut state);
    cleanup(&ctx, &state);
    result.expect("ship live journey failed");
}

#[derive(Default)]
struct State {
    product_id: String,
    suite_id: String,
    tag_id: String,
}

fn run_steps(ctx: &LiveCtx, s: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建产品（无删除接口，保留）。
    let name = ctx.unique_name("prod");
    let identifier = ctx.unique_identifier("SP");
    let product = ctx.run_ok(&[
        "ship",
        "product",
        "create",
        "--data",
        &json!({ "name": name, "identifier": identifier }).to_string(),
    ]);
    s.product_id = str_field(&product, "id")
        .map(str::to_string)
        .ok_or("create product has no id")?;
    assert_eq!(str_field(&product, "name"), Some(name.as_str()));
    eprintln!("created ship product {name} (id={})", s.product_id);

    // 2. 产品列表包含、详情一致。
    let list = ctx.run_ok(&[
        "ship",
        "product",
        "list",
        "--keywords",
        "pcl",
        "--page-size",
        "100",
    ]);
    assert!(find_by_id(&values(&list), &s.product_id).is_some());
    let fetched = ctx.run_ok(&["ship", "product", "get", &s.product_id]);
    assert_eq!(str_field(&fetched, "id"), Some(s.product_id.as_str()));

    // 3. 产品改名（update）。
    let renamed = ctx.unique_name("prod");
    let updated = ctx.run_ok(&[
        "ship",
        "product",
        "update",
        &s.product_id,
        "--data",
        &json!({ "name": renamed }).to_string(),
    ]);
    assert_eq!(str_field(&updated, "name"), Some(renamed.as_str()));

    // 4. 需求模块（suite）：create → list 包含 → get → delete（自清理）。
    let suite = ctx.run_ok(&[
        "ship",
        "product-suite",
        "create",
        &s.product_id,
        "--data",
        &json!({ "name": ctx.unique_name("suite"), "type": "module" }).to_string(),
    ]);
    s.suite_id = str_field(&suite, "id")
        .map(str::to_string)
        .ok_or("create suite has no id")?;
    let list = ctx.run_ok(&["ship", "product-suite", "list", &s.product_id]);
    assert!(find_by_id(&values(&list), &s.suite_id).is_some());
    ctx.run_ok(&["ship", "product-suite", "get", &s.product_id, &s.suite_id]);
    ctx.run_ok(&[
        "ship",
        "product-suite",
        "delete",
        &s.product_id,
        &s.suite_id,
    ]);
    s.suite_id.clear();
    eprintln!("product-suite CRUD verified");

    // 5. 标签（tag）：create → list 包含 → get → delete（自清理）。
    let tag = ctx.run_ok(&[
        "ship",
        "product-tag",
        "create",
        &s.product_id,
        "--data",
        &json!({ "name": ctx.unique_name("ptag") }).to_string(),
    ]);
    s.tag_id = str_field(&tag, "id")
        .map(str::to_string)
        .ok_or("create product-tag has no id")?;
    let list = ctx.run_ok(&["ship", "product-tag", "list", &s.product_id]);
    assert!(find_by_id(&values(&list), &s.tag_id).is_some());
    ctx.run_ok(&["ship", "product-tag", "get", &s.product_id, &s.tag_id]);
    ctx.run_ok(&["ship", "product-tag", "delete", &s.product_id, &s.tag_id]);
    s.tag_id.clear();
    eprintln!("product-tag CRUD verified");

    // 6. 产品成员：add → list 包含 → remove（有用户才做，不留残留）。
    if let Some(user_id) = support::first_user_id(ctx) {
        let body = json!({ "member": { "id": user_id, "type": "user" } });
        let membership = ctx.run_ok(&[
            "ship",
            "product-member",
            "add",
            &s.product_id,
            "--data",
            &body.to_string(),
        ]);
        let member_id = str_field(&membership, "id").unwrap_or(&user_id).to_string();
        let list = ctx.run_ok(&["ship", "product-member", "list", &s.product_id]);
        assert!(
            !values(&list).is_empty(),
            "product should have at least one member"
        );
        ctx.run_ok_ignored(&[
            "ship",
            "product-member",
            "remove",
            &s.product_id,
            &member_id,
        ]);
        eprintln!("product-member add/list/remove verified");
    } else {
        eprintln!("skip product-member steps: tenant has no users");
    }

    // 7. 需求（idea）：create → list 包含 → get → search 命中 → update（无删除，保留）。
    let idea_title = ctx.unique_name("idea");
    let idea = ctx.run_ok(&[
        "ship",
        "idea",
        "create",
        "--data",
        &json!({ "product_id": s.product_id, "title": idea_title }).to_string(),
    ]);
    let idea_id = str_field(&idea, "id").ok_or("create idea has no id")?;
    let list = ctx.run_ok(&["ship", "idea", "list", "--product-id", &s.product_id]);
    assert!(find_by_id(&values(&list), idea_id).is_some());
    let fetched = ctx.run_ok(&["ship", "idea", "get", idea_id]);
    assert_eq!(str_field(&fetched, "id"), Some(idea_id));
    let search = ctx.run_ok(&[
        "ship",
        "idea",
        "search",
        "--data",
        &json!({ "mode": "query", "payload": { "filter": { "product.id": { "in": [s.product_id.clone()] } } } })
            .to_string(),
    ]);
    assert!(find_by_id(&values(&search), idea_id).is_some());
    let idea_renamed = ctx.unique_name("idea");
    let updated = ctx.run_ok(&[
        "ship",
        "idea",
        "update",
        idea_id,
        "--data",
        &json!({ "title": idea_renamed }).to_string(),
    ]);
    assert_eq!(str_field(&updated, "title"), Some(idea_renamed.as_str()));
    eprintln!("idea create/list/get/search/update verified (kept, no delete API)");

    // 8. 工单（ticket）：需要 type_id（取产品工单类型字典第一个）；无类型则跳过。
    let types = ctx.run_ok(&["ship", "product-ticket-type", "list", &s.product_id]);
    if let Some(type_id) = values(&types).first().and_then(|t| str_field(t, "id")) {
        let ticket_title = ctx.unique_name("ticket");
        let ticket = ctx.run_ok(&[
            "ship",
            "ticket",
            "create",
            "--data",
            &json!({ "product_id": s.product_id, "title": ticket_title, "type_id": type_id })
                .to_string(),
        ]);
        let ticket_id = str_field(&ticket, "id").ok_or("create ticket has no id")?;
        let list = ctx.run_ok(&["ship", "ticket", "list", "--product-id", &s.product_id]);
        assert!(find_by_id(&values(&list), ticket_id).is_some());
        let fetched = ctx.run_ok(&["ship", "ticket", "get", ticket_id]);
        assert_eq!(str_field(&fetched, "id"), Some(ticket_id));
        let search = ctx.run_ok(&[
            "ship",
            "ticket",
            "search",
            "--data",
            &json!({ "mode": "query", "payload": { "filter": { "product.id": { "in": [s.product_id.clone()] } } } })
                .to_string(),
        ]);
        assert!(find_by_id(&values(&search), ticket_id).is_some());
        let ticket_renamed = ctx.unique_name("ticket");
        let updated = ctx.run_ok(&[
            "ship",
            "ticket",
            "update",
            ticket_id,
            "--data",
            &json!({ "title": ticket_renamed }).to_string(),
        ]);
        assert_eq!(str_field(&updated, "title"), Some(ticket_renamed.as_str()));
        eprintln!("ticket create/list/get/search/update verified (kept, no delete API)");
    } else {
        eprintln!("skip ticket steps: product has no ticket types");
    }

    Ok(())
}

fn cleanup(ctx: &LiveCtx, s: &State) {
    if ctx.keep {
        eprintln!("PC_LIVE_KEEP=1: skipping cleanup; product={}", s.product_id);
        return;
    }
    // suite/tag 正常流程已删除；这里兜底中途失败的残留。产品/需求/工单无删除接口。
    if !s.tag_id.is_empty() {
        ctx.run_ok_ignored(&["ship", "product-tag", "delete", &s.product_id, &s.tag_id]);
    }
    if !s.suite_id.is_empty() {
        ctx.run_ok_ignored(&[
            "ship",
            "product-suite",
            "delete",
            &s.product_id,
            &s.suite_id,
        ]);
    }
    if !s.product_id.is_empty() {
        eprintln!(
            "product {} (and its ideas/tickets) have no delete API; kept with pcl- prefix",
            s.product_id
        );
    }
}
