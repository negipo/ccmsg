use crate::db::Database;
use anyhow::Result;
use std::io::{self, Write};

pub fn run(yes: bool) -> Result<()> {
    if !yes && !confirm()? {
        eprintln!("Aborted");
        return Ok(());
    }
    let db = Database::open()?;
    db.reset()?;
    eprintln!("ccmsg state has been reset");
    Ok(())
}

fn confirm() -> Result<bool> {
    eprint!("This will delete all ccmsg messages and peers. Continue? [y/N] ");
    io::stderr().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let answer = input.trim().to_lowercase();
    Ok(answer == "y" || answer == "yes")
}
