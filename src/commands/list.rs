use crate::db::Database;
use anyhow::Result;

pub fn run() -> Result<()> {
    let db = Database::open()?;
    let names = db.list_agents()?;
    if names.is_empty() {
        println!("既知の宛先はありません");
        return Ok(());
    }
    println!("既知の宛先 {} 件:", names.len());
    for n in names {
        println!("- {}", n);
    }
    Ok(())
}
