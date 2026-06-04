use ccmsg::commands::hooks::{
    load_settings, register_hook, save_settings, unregister_hook, HOOK_COMMAND,
};
use serde_json::json;
use tempfile::TempDir;

#[test]
fn test_register_into_empty_settings() {
    let mut s = json!({});
    assert!(register_hook(&mut s));
    assert_eq!(
        s["hooks"]["SessionStart"][0]["hooks"][0]["command"],
        HOOK_COMMAND
    );
}

#[test]
fn test_register_is_idempotent() {
    let mut s = json!({});
    assert!(register_hook(&mut s));
    assert!(!register_hook(&mut s));
    let arr = s["hooks"]["SessionStart"].as_array().unwrap();
    assert_eq!(arr.len(), 1);
}

#[test]
fn test_register_preserves_existing_settings() {
    let mut s = json!({
        "model": "opus",
        "hooks": { "Stop": [ { "hooks": [ { "type": "command", "command": "other" } ] } ] }
    });
    register_hook(&mut s);
    assert_eq!(s["model"], "opus");
    assert_eq!(s["hooks"]["Stop"][0]["hooks"][0]["command"], "other");
    assert_eq!(
        s["hooks"]["SessionStart"][0]["hooks"][0]["command"],
        HOOK_COMMAND
    );
}

#[test]
fn test_unregister_removes_only_our_hook() {
    let mut s = json!({});
    register_hook(&mut s);
    assert!(unregister_hook(&mut s));
    assert!(s.get("hooks").is_none());
}

#[test]
fn test_unregister_keeps_other_session_start_hooks() {
    let mut s = json!({
        "hooks": { "SessionStart": [ { "hooks": [ { "type": "command", "command": "keepme" } ] } ] }
    });
    register_hook(&mut s);
    assert!(unregister_hook(&mut s));
    let arr = s["hooks"]["SessionStart"].as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["hooks"][0]["command"], "keepme");
}

#[test]
fn test_unregister_when_absent_returns_false() {
    let mut s = json!({ "model": "opus" });
    assert!(!unregister_hook(&mut s));
}

#[test]
fn test_load_missing_returns_empty_object() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("settings.json");
    assert_eq!(load_settings(&path).unwrap(), json!({}));
}

#[test]
fn test_save_then_load_roundtrip() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("settings.json");
    let mut s = json!({});
    register_hook(&mut s);
    save_settings(&path, &s).unwrap();
    assert_eq!(load_settings(&path).unwrap(), s);
}
