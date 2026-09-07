//! 旅程 J：testhub 测试库（library）+ 测试套件（library-suite）+ 测试用例（testcase）。
//!
//! 测试库没有删除接口（创建后保留，pcl 前缀便于人工清理）；套件和用例有删除接口，
//! 完整自清理。

use serde_json::json;

use crate::common::{find_by_id, str_field, values, LiveCtx};

#[test]
fn journey_testhub_library_testcase() {
    let Some(ctx) = LiveCtx::new() else {
        return;
    };

    let mut state = State::default();
    let result = run_steps(&ctx, &mut state);
    cleanup(&ctx, &state);
    result.expect("testhub live journey failed");
}

#[derive(Default)]
struct State {
    library_id: String,
    suite_id: String,
    testcase_id: String,
}

fn run_steps(ctx: &LiveCtx, s: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建测试库（无删除接口，保留）。
    let name = ctx.unique_name("lib");
    let identifier = ctx.unique_identifier("TL");
    let library = ctx.run_ok(&[
        "testhub",
        "library",
        "create",
        "--data",
        &json!({ "name": name, "identifier": identifier }).to_string(),
    ]);
    s.library_id = str_field(&library, "id")
        .map(str::to_string)
        .ok_or("create library has no id")?;
    assert_eq!(str_field(&library, "name"), Some(name.as_str()));
    eprintln!("created testhub library {name} (id={})", s.library_id);

    // 2. 库列表包含、详情一致、改名。
    let list = ctx.run_ok(&["testhub", "library", "list", "--keywords", "pcl"]);
    assert!(find_by_id(&values(&list), &s.library_id).is_some());
    let fetched = ctx.run_ok(&["testhub", "library", "get", &s.library_id]);
    assert_eq!(str_field(&fetched, "id"), Some(s.library_id.as_str()));
    let renamed = ctx.unique_name("lib");
    let updated = ctx.run_ok(&[
        "testhub",
        "library",
        "update",
        &s.library_id,
        "--data",
        &json!({ "name": renamed }).to_string(),
    ]);
    assert_eq!(str_field(&updated, "name"), Some(renamed.as_str()));

    // 3. 测试套件（suite）：create → list 包含 → get → update → delete（自清理）。
    let suite = ctx.run_ok(&[
        "testhub",
        "library-suite",
        "create",
        &s.library_id,
        "--data",
        &json!({ "name": ctx.unique_name("suite") }).to_string(),
    ]);
    s.suite_id = str_field(&suite, "id")
        .map(str::to_string)
        .ok_or("create suite has no id")?;
    let list = ctx.run_ok(&["testhub", "library-suite", "list", &s.library_id]);
    assert!(find_by_id(&values(&list), &s.suite_id).is_some());
    ctx.run_ok(&[
        "testhub",
        "library-suite",
        "get",
        &s.library_id,
        &s.suite_id,
    ]);
    let suite_renamed = ctx.unique_name("suite");
    ctx.run_ok(&[
        "testhub",
        "library-suite",
        "update",
        &s.library_id,
        &s.suite_id,
        "--data",
        &json!({ "name": suite_renamed }).to_string(),
    ]);
    ctx.run_ok(&[
        "testhub",
        "library-suite",
        "remove",
        &s.library_id,
        &s.suite_id,
    ]);
    s.suite_id.clear();
    eprintln!("library-suite CRUD verified");

    // 4. 测试用例（testcase）：create → list 包含 → get → search 命中 → histories → update → delete。
    let case_title = ctx.unique_name("case");
    let testcase = ctx.run_ok(&[
        "testhub",
        "testcase",
        "create",
        "--data",
        &json!({ "library_id": s.library_id, "title": case_title }).to_string(),
    ]);
    s.testcase_id = str_field(&testcase, "id")
        .map(str::to_string)
        .ok_or("create testcase has no id")?;
    assert_eq!(str_field(&testcase, "title"), Some(case_title.as_str()));
    eprintln!("created testcase {case_title} (id={})", s.testcase_id);

    let list = ctx.run_ok(&["testhub", "testcase", "list", "--library-id", &s.library_id]);
    assert!(find_by_id(&values(&list), &s.testcase_id).is_some());
    let fetched = ctx.run_ok(&["testhub", "testcase", "get", &s.testcase_id]);
    assert_eq!(str_field(&fetched, "id"), Some(s.testcase_id.as_str()));

    let search = ctx.run_ok(&[
        "testhub",
        "testcase",
        "search",
        "--data",
        &json!({ "mode": "query", "payload": { "filter": { "library.id": { "in": [s.library_id.clone()] } } } })
            .to_string(),
    ]);
    assert!(find_by_id(&values(&search), &s.testcase_id).is_some());

    // histories 冒烟（只读）。
    let histories = ctx.run_ok(&["testhub", "testcase", "histories", &s.testcase_id]);
    let _ = values(&histories);

    let case_renamed = ctx.unique_name("case");
    let updated = ctx.run_ok(&[
        "testhub",
        "testcase",
        "update",
        &s.testcase_id,
        "--data",
        &json!({ "title": case_renamed }).to_string(),
    ]);
    assert_eq!(str_field(&updated, "title"), Some(case_renamed.as_str()));

    // 5. 负面用例：不存在的用例应失败。
    let stderr = ctx.run_fail(&["testhub", "testcase", "get", "pcl-nope-case"]);
    assert!(!stderr.is_empty());

    // 6. 删除用例，之后 get 应失败（库无删除接口，保留）。
    ctx.run_ok(&["testhub", "testcase", "delete", &s.testcase_id]);
    ctx.run_fail(&["testhub", "testcase", "get", &s.testcase_id]);
    s.testcase_id.clear();
    eprintln!("testcase CRUD verified (library kept, no delete API)");

    Ok(())
}

fn cleanup(ctx: &LiveCtx, s: &State) {
    if ctx.keep {
        eprintln!("PC_LIVE_KEEP=1: skipping cleanup; library={}", s.library_id);
        return;
    }
    if !s.testcase_id.is_empty() {
        ctx.run_ok_ignored(&["testhub", "testcase", "delete", &s.testcase_id]);
    }
    if !s.suite_id.is_empty() {
        ctx.run_ok_ignored(&[
            "testhub",
            "library-suite",
            "remove",
            &s.library_id,
            &s.suite_id,
        ]);
    }
    if !s.library_id.is_empty() {
        eprintln!(
            "library {} has no delete API; kept with pcl- prefix for manual cleanup",
            s.library_id
        );
    }
}
