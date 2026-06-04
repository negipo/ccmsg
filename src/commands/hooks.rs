use anyhow::Result;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

/// settings.json に登録する hook コマンド
pub const HOOK_COMMAND: &str = "ccmsg hook-session-start";

/// ~/.claude/settings.json のパス
pub fn settings_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join(".claude")
        .join("settings.json")
}

/// settings へ SessionStart hook を登録する。既に登録済みなら false を返す。
pub fn register_hook(settings: &mut Value) -> bool {
    if !settings.is_object() {
        *settings = json!({});
    }
    let obj = settings.as_object_mut().unwrap();
    let hooks = obj.entry("hooks").or_insert_with(|| json!({}));
    if !hooks.is_object() {
        *hooks = json!({});
    }
    let hooks_obj = hooks.as_object_mut().unwrap();
    let session_start = hooks_obj.entry("SessionStart").or_insert_with(|| json!([]));
    if !session_start.is_array() {
        *session_start = json!([]);
    }
    let arr = session_start.as_array_mut().unwrap();
    if arr.iter().any(group_has_our_hook) {
        return false;
    }
    arr.push(json!({
        "hooks": [ { "type": "command", "command": HOOK_COMMAND } ]
    }));
    true
}

/// settings から自分の SessionStart hook を除去する。除去したら true を返す。
pub fn unregister_hook(settings: &mut Value) -> bool {
    let Some(obj) = settings.as_object_mut() else {
        return false;
    };
    let mut removed = false;
    {
        let Some(hooks) = obj.get_mut("hooks").and_then(|h| h.as_object_mut()) else {
            return false;
        };
        let Some(session_start) = hooks.get_mut("SessionStart").and_then(|s| s.as_array_mut())
        else {
            return false;
        };
        for group in session_start.iter_mut() {
            if let Some(inner) = group.get_mut("hooks").and_then(|h| h.as_array_mut()) {
                let before = inner.len();
                inner.retain(|h| !is_our_hook(h));
                if inner.len() != before {
                    removed = true;
                }
            }
        }
        session_start.retain(|group| {
            group
                .get("hooks")
                .and_then(|h| h.as_array())
                .is_none_or(|a| !a.is_empty())
        });
    }
    if let Some(hooks) = obj.get_mut("hooks").and_then(|h| h.as_object_mut()) {
        let ss_empty = hooks
            .get("SessionStart")
            .and_then(|s| s.as_array())
            .is_some_and(|a| a.is_empty());
        if ss_empty {
            hooks.remove("SessionStart");
        }
        if hooks.is_empty() {
            obj.remove("hooks");
        }
    }
    removed
}

fn group_has_our_hook(group: &Value) -> bool {
    group
        .get("hooks")
        .and_then(|h| h.as_array())
        .is_some_and(|arr| arr.iter().any(is_our_hook))
}

fn is_our_hook(h: &Value) -> bool {
    h.get("command").and_then(|c| c.as_str()) == Some(HOOK_COMMAND)
}

/// settings.json を読み込む。無い／空なら空オブジェクトを返す。
pub fn load_settings(path: &Path) -> Result<Value> {
    if !path.exists() {
        return Ok(json!({}));
    }
    let content = fs::read_to_string(path)?;
    if content.trim().is_empty() {
        return Ok(json!({}));
    }
    Ok(serde_json::from_str(&content)?)
}

/// settings.json を pretty 形式で書き込む。
pub fn save_settings(path: &Path, settings: &Value) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut s = serde_json::to_string_pretty(settings)?;
    s.push('\n');
    fs::write(path, s)?;
    Ok(())
}
