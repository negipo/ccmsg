use ccmsg::commands::hook_session_start::{append_export, export_line};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_export_line_quotes_value() {
    assert_eq!(
        export_line("/p/repo-a"),
        "export CCMSG_PROJECT_DIR='/p/repo-a'\n"
    );
}

#[test]
fn test_export_line_escapes_single_quote() {
    assert_eq!(
        export_line("/p/o'brien"),
        "export CCMSG_PROJECT_DIR='/p/o'\\''brien'\n"
    );
}

#[test]
fn test_append_export_appends_line() {
    let tmp = TempDir::new().unwrap();
    let env_file = tmp.path().join("env");
    append_export(env_file.to_str().unwrap(), "/p/repo-a").unwrap();
    let content = fs::read_to_string(&env_file).unwrap();
    assert_eq!(content, "export CCMSG_PROJECT_DIR='/p/repo-a'\n");
}
