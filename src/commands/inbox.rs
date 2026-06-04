use crate::db::{basename, Database};
use crate::models::{Message, ReceiveOutcome};
use anyhow::Result;

pub fn run(project: &str) -> Result<()> {
    let name = basename(project);
    let db = Database::open()?;
    match db.receive_once(&name, project)? {
        ReceiveOutcome::Claimed(msgs) => print_messages(&name, &msgs),
        ReceiveOutcome::Collision {
            existing_path,
            unread,
        } => {
            print_collision(&name, &existing_path);
            print_messages(&name, &unread);
            eprintln!("（衝突のため既読化を保留しました）");
        }
    }
    Ok(())
}

pub fn print_messages(name: &str, msgs: &[Message]) {
    if msgs.is_empty() {
        println!("{} 宛の未読はありません", name);
        return;
    }
    println!("{} 宛の未読 {} 件:", name, msgs.len());
    for m in msgs {
        println!("- [{}] {}: {}", m.created_at, m.from_agent, m.body);
    }
}

pub fn print_collision(name: &str, existing_path: &str) {
    eprintln!(
        "警告: identity '{}' は別のパス ({}) で既に登録されています。どちらかのディレクトリ名を変えて解消してください",
        name, existing_path
    );
}
