mod board;
mod board_entry;
mod board_swimlane;
mod deliverable;
mod project;
mod project_member;
mod project_process;
mod project_property;
mod project_state;
mod release;
mod release_category;
mod release_section;
mod release_stage;
mod workitem;
mod workitem_priority;
mod workitem_property;
mod workitem_property_plan;
mod workitem_relation;
mod workitem_relation_type;
mod workitem_state;
mod workitem_state_plan;
mod workitem_tag;
mod workitem_transition;
mod workitem_type;
mod workitem_type_plan;

use crate::common::pc;
use predicates::prelude::*;

#[test]
fn pjm_help_lists_workitem_resource() {
    pc().arg("pjm")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("workitem"));
}
