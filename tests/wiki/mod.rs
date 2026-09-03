mod page;
mod space;
mod space_member;

use crate::common::pc;
use predicates::prelude::*;

#[test]
fn wiki_help_lists_space_and_page_resources() {
    pc().arg("wiki")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("space"))
        .stdout(predicate::str::contains("space-member"))
        .stdout(predicate::str::contains("page"));
}
