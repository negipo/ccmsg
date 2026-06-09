use crate::db::{resolve_identity, Database};
use crate::models::{Message, ReceiveOutcome};
use anyhow::Result;

pub fn run(project: &str, history: Option<usize>) -> Result<()> {
    let name = resolve_identity(project)?;
    let db = Database::open()?;
    if let Some(limit) = history {
        print_history(&name, &db.recent_read(&name, limit)?);
        return Ok(());
    }
    match db.receive_once(&name, project)? {
        ReceiveOutcome::Claimed(msgs) => print_messages(&name, &msgs),
        ReceiveOutcome::Collision {
            existing_path,
            unread,
        } => {
            print_collision(&name, &existing_path);
            print_messages(&name, &unread);
            eprintln!("(marking-as-read held back due to collision)");
        }
    }
    Ok(())
}

pub fn print_messages(name: &str, msgs: &[Message]) {
    if msgs.is_empty() {
        println!("No unread messages for {}", name);
        return;
    }
    println!("{} unread message(s) for {}:", msgs.len(), name);
    for m in msgs {
        println!("- [{}] {}: {}", m.created_at, m.from_agent, m.body);
    }
}

pub fn print_history(name: &str, msgs: &[Message]) {
    if msgs.is_empty() {
        println!("No read messages for {}", name);
        return;
    }
    println!(
        "{} read message(s) for {} (most recent first):",
        msgs.len(),
        name
    );
    for m in msgs {
        println!("- [{}] {}: {}", m.created_at, m.from_agent, m.body);
    }
}

pub fn print_collision(name: &str, existing_path: &str) {
    eprintln!(
        "warning: identity '{}' is already registered under a different path ({}). Resolve it by renaming one of the directories",
        name, existing_path
    );
}
