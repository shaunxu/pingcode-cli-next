mod product;
mod product_channel;
mod product_customer;
mod product_member;
mod product_plan;
mod product_suite;
mod product_tag;
mod product_ticket_type;
mod product_user;

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
