mod idea;
mod idea_plan;
mod idea_priority;
mod idea_property;
mod idea_property_plan;
mod idea_state;
mod idea_suite;
mod idea_transition;
mod product;
mod product_channel;
mod product_customer;
mod product_member;
mod product_plan;
mod product_suite;
mod product_tag;
mod product_ticket_type;
mod product_user;
mod ticket;
mod ticket_channel;
mod ticket_priority;
mod ticket_property;
mod ticket_property_plan;
mod ticket_solution;
mod ticket_state;
mod ticket_state_plan;
mod ticket_tag;
mod ticket_transition;
mod ticket_type;

use crate::common::pc;
use predicates::prelude::*;

#[test]
fn ship_help_lists_product_resource() {
    pc().arg("ship")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("product"));
}
