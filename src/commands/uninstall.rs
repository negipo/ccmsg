use crate::commands::install::skills_install_dir;
use anyhow::Result;
use std::fs;

const SKILL_NAMES: &[&str] = &["ccmsg", "ccmsg-sending", "ccmsg-listing-peers"];

pub fn run() -> Result<()> {
    let skills_dir = skills_install_dir();
    for name in SKILL_NAMES {
        let dir = skills_dir.join(name);
        if dir.exists() {
            fs::remove_dir_all(&dir)?;
            eprintln!("Removed: {}", dir.display());
        }
    }
    eprintln!("Skills uninstalled from {}", skills_dir.display());
    Ok(())
}
