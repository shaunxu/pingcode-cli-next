//! 旅程 D：pjm 看板（board）+ 看板栏（entry）+ 泳道（swimlane）全套 CRUD。
//! 全部资源都有删除接口，完整自清理；共享测试项目（hybrid）无删除接口，保留。

use serde_json::json;

use crate::common::{find_by_id, str_field, values, LiveCtx};
use crate::journeys::support;

#[test]
fn journey_pjm_board_entry_swimlane() {
    let Some(ctx) = LiveCtx::new() else {
        return;
    };

    let mut state = State::default();
    let result = run_steps(&ctx, &mut state);
    cleanup(&ctx, &state);
    result.expect("board live journey failed");
}

#[derive(Default)]
struct State {
    project_id: String,
    board_id: String,
    entry_id: String,
    swimlane_id: String,
}

fn run_steps(ctx: &LiveCtx, s: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    s.project_id = support::shared_project_id(ctx);

    // 1. 创建看板。
    let board_name = ctx.unique_name("board");
    let board = ctx.run_ok(&[
        "pjm",
        "board",
        "create",
        &s.project_id,
        "--data",
        &json!({ "name": board_name }).to_string(),
    ]);
    s.board_id = str_field(&board, "id")
        .map(str::to_string)
        .ok_or("create board response has no id")?;
    assert_eq!(str_field(&board, "name"), Some(board_name.as_str()));
    eprintln!("created board {board_name} (id={})", s.board_id);

    // 2. 看板列表包含。
    let list = ctx.run_ok(&["pjm", "board", "list", &s.project_id]);
    assert!(find_by_id(&values(&list), &s.board_id).is_some());

    // 3. 看板详情，改名后详情反映。
    let fetched = ctx.run_ok(&["pjm", "board", "get", &s.project_id, &s.board_id]);
    assert_eq!(str_field(&fetched, "id"), Some(s.board_id.as_str()));
    let board_renamed = ctx.unique_name("board");
    let updated = ctx.run_ok(&[
        "pjm",
        "board",
        "update",
        &s.project_id,
        &s.board_id,
        "--data",
        &json!({ "name": board_renamed }).to_string(),
    ]);
    assert_eq!(str_field(&updated, "name"), Some(board_renamed.as_str()));

    // 4. 创建看板栏（entry），list/get 交叉验证，改名。
    let entry_name = ctx.unique_name("entry");
    let entry = ctx.run_ok(&[
        "pjm",
        "board-entry",
        "create",
        &s.project_id,
        &s.board_id,
        "--data",
        &json!({ "name": entry_name, "wip_limit": 5 }).to_string(),
    ]);
    s.entry_id = str_field(&entry, "id")
        .map(str::to_string)
        .ok_or("create entry response has no id")?;
    let list = ctx.run_ok(&["pjm", "board-entry", "list", &s.project_id, &s.board_id]);
    assert!(find_by_id(&values(&list), &s.entry_id).is_some());
    let fetched = ctx.run_ok(&[
        "pjm",
        "board-entry",
        "get",
        &s.project_id,
        &s.board_id,
        &s.entry_id,
    ]);
    assert_eq!(str_field(&fetched, "id"), Some(s.entry_id.as_str()));
    let entry_renamed = ctx.unique_name("entry");
    ctx.run_ok(&[
        "pjm",
        "board-entry",
        "update",
        &s.project_id,
        &s.board_id,
        &s.entry_id,
        "--data",
        &json!({ "name": entry_renamed }).to_string(),
    ]);

    // 5. 创建泳道（swimlane），list/get 交叉验证，改名。
    let lane_name = ctx.unique_name("lane");
    let lane = ctx.run_ok(&[
        "pjm",
        "board-swimlane",
        "create",
        &s.project_id,
        &s.board_id,
        "--data",
        &json!({ "name": lane_name }).to_string(),
    ]);
    s.swimlane_id = str_field(&lane, "id")
        .map(str::to_string)
        .ok_or("create swimlane response has no id")?;
    let list = ctx.run_ok(&["pjm", "board-swimlane", "list", &s.project_id, &s.board_id]);
    assert!(find_by_id(&values(&list), &s.swimlane_id).is_some());
    ctx.run_ok(&[
        "pjm",
        "board-swimlane",
        "get",
        &s.project_id,
        &s.board_id,
        &s.swimlane_id,
    ]);
    let lane_renamed = ctx.unique_name("lane");
    ctx.run_ok(&[
        "pjm",
        "board-swimlane",
        "update",
        &s.project_id,
        &s.board_id,
        &s.swimlane_id,
        "--data",
        &json!({ "name": lane_renamed }).to_string(),
    ]);

    // 6. 负面用例：不存在的看板应失败。
    let stderr = ctx.run_fail(&["pjm", "board", "get", &s.project_id, "pcl-nope-board"]);
    assert!(!stderr.is_empty());

    // 7. 删除看板栏与泳道，随后删除看板；删除后 get 均应失败。
    ctx.run_ok(&[
        "pjm",
        "board-entry",
        "delete",
        &s.project_id,
        &s.board_id,
        &s.entry_id,
    ]);
    ctx.run_ok(&[
        "pjm",
        "board-swimlane",
        "delete",
        &s.project_id,
        &s.board_id,
        &s.swimlane_id,
    ]);
    ctx.run_ok(&["pjm", "board", "delete", &s.project_id, &s.board_id]);
    ctx.run_fail(&["pjm", "board", "get", &s.project_id, &s.board_id]);
    s.board_id.clear();
    s.entry_id.clear();
    s.swimlane_id.clear();
    eprintln!("deleted board/entry/swimlane (journey complete)");

    Ok(())
}

fn cleanup(ctx: &LiveCtx, s: &State) {
    if ctx.keep {
        eprintln!("PC_LIVE_KEEP=1: skipping cleanup; board={}", s.board_id);
        return;
    }
    if !s.entry_id.is_empty() {
        ctx.run_ok_ignored(&[
            "pjm",
            "board-entry",
            "delete",
            &s.project_id,
            &s.board_id,
            &s.entry_id,
        ]);
    }
    if !s.swimlane_id.is_empty() {
        ctx.run_ok_ignored(&[
            "pjm",
            "board-swimlane",
            "delete",
            &s.project_id,
            &s.board_id,
            &s.swimlane_id,
        ]);
    }
    if !s.board_id.is_empty() {
        ctx.run_ok_ignored(&["pjm", "board", "delete", &s.project_id, &s.board_id]);
    }
}
