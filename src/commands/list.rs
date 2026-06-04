use crate::db::Database;
use anyhow::Result;

pub fn run() -> Result<()> {
    let db = Database::open()?;
    let names = db.list_agents()?;
    if names.is_empty() {
        println!("No known peers");
        return Ok(());
    }
    println!("{} known peer(s):", names.len());
    for n in names {
        println!("- {}", n);
    }
    Ok(())
}
