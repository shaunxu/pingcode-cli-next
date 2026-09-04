use crate::common::pc;
use predicates::prelude::*;

#[test]
fn attachments_help_lists_operations() {
    pc().arg("attachments")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("upload-file"))
        .stdout(predicate::str::contains("upload-snippet"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_attachments_list_previews_query() {
    pc().arg("--dry-run")
        .arg("attachments")
        .arg("list")
        .arg("--principal-type")
        .arg("workitem")
        .arg("--principal-id")
        .arg("wi-123")
        .arg("--comment-id")
        .arg("cmt-9")
        .arg("--page-index")
        .arg("0")
        .arg("--page-size")
        .arg("50")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/attachments?",
        ))
        .stderr(predicate::str::contains("principal_type=workitem"))
        .stderr(predicate::str::contains("principal_id=wi-123"))
        .stderr(predicate::str::contains("comment_id=cmt-9"))
        .stderr(predicate::str::contains("page_index=0"))
        .stderr(predicate::str::contains("page_size=50"));
}

#[test]
fn dry_run_attachments_get_previews_query() {
    pc().arg("--dry-run")
        .arg("attachments")
        .arg("get")
        .arg("att-1")
        .arg("--principal-type")
        .arg("idea")
        .arg("--principal-id")
        .arg("idea-7")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/attachments/att-1?",
        ))
        .stderr(predicate::str::contains("principal_type=idea"))
        .stderr(predicate::str::contains("principal_id=idea-7"));
}

#[test]
fn dry_run_attachments_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("attachments")
        .arg("delete")
        .arg("att-2")
        .arg("--principal-type")
        .arg("ticket")
        .arg("--principal-id")
        .arg("t-3")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/attachments/att-2?",
        ))
        .stderr(predicate::str::contains("principal_type=ticket"));
}

#[test]
fn dry_run_attachments_upload_snippet_previews_post_body() {
    pc().arg("--dry-run")
        .arg("attachments")
        .arg("upload-snippet")
        .arg("--data")
        .arg(
            r#"{"principal_type":"workitem","principal_id":"wi-1","title":"snippet.rs","format":"rust","content":"fn main() {}"}"#,
        )
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/attachments",
        ))
        .stderr(predicate::str::contains("snippet.rs"))
        .stderr(predicate::str::contains("fn main"));
}

#[test]
fn dry_run_attachments_upload_file_previews_multipart() {
    pc().arg("--dry-run")
        .arg("attachments")
        .arg("upload-file")
        .arg("--principal-type")
        .arg("page")
        .arg("--principal-id")
        .arg("pg-1")
        .arg("--file")
        .arg("/tmp/demo-upload.png")
        .arg("--title")
        .arg("demo.png")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/attachments?",
        ))
        .stderr(predicate::str::contains("principal_type=page"))
        .stderr(predicate::str::contains("principal_id=pg-1"))
        .stderr(predicate::str::contains("multipart_form"))
        .stderr(predicate::str::contains("demo.png"))
        .stderr(predicate::str::contains("@/tmp/demo-upload.png"));
}

#[test]
fn attachments_list_requires_principal_type_and_id() {
    pc().arg("--dry-run")
        .arg("attachments")
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--principal-type"))
        .stderr(predicate::str::contains("--principal-id"));
}

#[test]
fn attachments_list_rejects_unknown_principal_type() {
    pc().arg("--dry-run")
        .arg("attachments")
        .arg("list")
        .arg("--principal-type")
        .arg("bogus")
        .arg("--principal-id")
        .arg("x")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}
