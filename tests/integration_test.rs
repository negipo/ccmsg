use std::process::Command;
use std::sync::Once;
use tempfile::TempDir;

static BUILD: Once = Once::new();

fn ccmsg_bin() -> String {
    BUILD.call_once(|| {
        let status = Command::new("cargo").args(["build"]).status().unwrap();
        assert!(status.success());
    });
    let output = Command::new("cargo")
        .args(["metadata", "--format-version=1", "--no-deps"])
        .output()
        .unwrap();
    let meta: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let target_dir = meta["target_directory"].as_str().unwrap();
    format!("{}/debug/ccmsg", target_dir)
}

fn run(bin: &str, db: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(bin)
        .args(args)
        .env("CCMSG_DB_PATH", db)
        .output()
        .unwrap()
}

#[test]
fn test_send_then_inbox_roundtrip() {
    let bin = ccmsg_bin();
    let tmp = TempDir::new().unwrap();
    let db = tmp.path().join("messages.db");

    run(&bin, &db, &["inbox", "--project", "/p/alpha"]);
    run(&bin, &db, &["inbox", "--project", "/p/beta"]);

    let sent = run(
        &bin,
        &db,
        &[
            "send",
            "--to",
            "beta",
            "--body",
            "hello-beta",
            "--project",
            "/p/alpha",
        ],
    );
    assert!(sent.status.success());

    let inbox = run(&bin, &db, &["inbox", "--project", "/p/beta"]);
    let stdout = String::from_utf8_lossy(&inbox.stdout);
    assert!(stdout.contains("hello-beta"));
    assert!(stdout.contains("alpha"));

    let inbox2 = run(&bin, &db, &["inbox", "--project", "/p/beta"]);
    let stdout2 = String::from_utf8_lossy(&inbox2.stdout);
    assert!(stdout2.contains("未読はありません"));
}

#[test]
fn test_send_to_unknown_fails() {
    let bin = ccmsg_bin();
    let tmp = TempDir::new().unwrap();
    let db = tmp.path().join("messages.db");

    run(&bin, &db, &["inbox", "--project", "/p/alpha"]);
    let sent = run(
        &bin,
        &db,
        &[
            "send",
            "--to",
            "ghost",
            "--body",
            "x",
            "--project",
            "/p/alpha",
        ],
    );
    assert!(!sent.status.success());
}

#[test]
fn test_list_shows_registered_agents() {
    let bin = ccmsg_bin();
    let tmp = TempDir::new().unwrap();
    let db = tmp.path().join("messages.db");

    run(&bin, &db, &["inbox", "--project", "/p/alpha"]);
    run(&bin, &db, &["inbox", "--project", "/p/beta"]);
    let listed = run(&bin, &db, &["list"]);
    let stdout = String::from_utf8_lossy(&listed.stdout);
    assert!(stdout.contains("alpha"));
    assert!(stdout.contains("beta"));
}

#[test]
fn test_wait_times_out_without_messages() {
    let bin = ccmsg_bin();
    let tmp = TempDir::new().unwrap();
    let db = tmp.path().join("messages.db");

    let waited = run(
        &bin,
        &db,
        &["wait", "--project", "/p/alpha", "--timeout", "1"],
    );
    assert!(waited.status.success());
    let stdout = String::from_utf8_lossy(&waited.stdout);
    assert!(stdout.contains("タイムアウト"));
}

#[test]
fn test_wait_returns_preexisting_unread_immediately() {
    let bin = ccmsg_bin();
    let tmp = TempDir::new().unwrap();
    let db = tmp.path().join("messages.db");

    run(&bin, &db, &["inbox", "--project", "/p/alpha"]);
    run(&bin, &db, &["inbox", "--project", "/p/beta"]);
    run(
        &bin,
        &db,
        &[
            "send",
            "--to",
            "beta",
            "--body",
            "queued",
            "--project",
            "/p/alpha",
        ],
    );

    let waited = run(
        &bin,
        &db,
        &["wait", "--project", "/p/beta", "--timeout", "30"],
    );
    let stdout = String::from_utf8_lossy(&waited.stdout);
    assert!(stdout.contains("queued"));
}
