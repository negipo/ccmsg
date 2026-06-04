use crate::db::{basename, Database};
use crate::models::RegisterOutcome;
use anyhow::{bail, Result};

pub fn run(to: &str, body: &str, project: &str) -> Result<()> {
    let from = basename(project);
    let db = Database::open()?;

    if let RegisterOutcome::Collision { existing_path } = db.register_self(&from, project)? {
        bail!(
            "identity '{}' が別のパス ({}) と衝突しています。誤配信を防ぐため送信を中止しました",
            from,
            existing_path
        );
    }

    db.send_message(&from, to, body)?;
    println!("{} -> {}: 送信しました", from, to);
    Ok(())
}
