use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;

/// CLAUDE_ENV_FILE へ追記する export 行を作る
pub fn export_line(project_dir: &str) -> String {
    format!("export CCMSG_PROJECT_DIR={}\n", shell_quote(project_dir))
}

/// シェルで安全に展開されるよう値をシングルクォートで囲む
fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// env file へ export 行を追記する（無ければ作成）
pub fn append_export(env_file: &str, project_dir: &str) -> Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(env_file)?;
    f.write_all(export_line(project_dir).as_bytes())?;
    Ok(())
}

/// SessionStart hook の本体。
/// hook 環境の $CLAUDE_PROJECT_DIR を $CLAUDE_ENV_FILE へ退避する。
/// どちらかが空なら何もせず正常終了する（stdin は読まない）。
pub fn run() -> Result<()> {
    let env_file = std::env::var("CLAUDE_ENV_FILE").unwrap_or_default();
    let project_dir = std::env::var("CLAUDE_PROJECT_DIR").unwrap_or_default();
    if env_file.is_empty() || project_dir.is_empty() {
        return Ok(());
    }
    append_export(&env_file, &project_dir)?;
    Ok(())
}
