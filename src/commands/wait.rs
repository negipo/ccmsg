use crate::commands::inbox::{print_collision, print_messages};
use crate::db::{basename, Database};
use crate::models::ReceiveOutcome;
use anyhow::Result;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn run(project: &str, timeout: u64) -> Result<()> {
    let name = basename(project);
    let db = Database::open()?;
    let deadline = Instant::now() + Duration::from_secs(timeout);

    loop {
        match db.receive_once(&name, project)? {
            ReceiveOutcome::Collision {
                existing_path,
                unread,
            } => {
                print_collision(&name, &existing_path);
                print_messages(&name, &unread);
                eprintln!("（衝突のため既読化を保留しました）");
                return Ok(());
            }
            ReceiveOutcome::Claimed(msgs) if !msgs.is_empty() => {
                print_messages(&name, &msgs);
                return Ok(());
            }
            ReceiveOutcome::Claimed(_) => {
                if Instant::now() >= deadline {
                    println!("新着はありませんでした（タイムアウト {} 秒）", timeout);
                    return Ok(());
                }
                sleep(Duration::from_secs(1));
            }
        }
    }
}
