use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ccmsg", about = "Messaging between Claude Code sessions")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 自分宛の未読を表示・既読化する（agents 簿へ自己登録）
    Inbox {
        #[arg(long)]
        project: String,
    },
    /// 新着が来るまでブロックし、到着で表示・既読化する
    Wait {
        #[arg(long)]
        project: String,
        #[arg(long, default_value = "60")]
        timeout: u64,
    },
    /// 宛先へメッセージを送る（from は project の basename）
    Send {
        #[arg(long)]
        to: String,
        #[arg(long)]
        body: String,
        #[arg(long)]
        project: String,
    },
    /// 既知の宛先（ディレクトリ名）一覧を表示する
    List,
    /// skill を ~/.claude/skills/ へ配置する
    Install,
    /// skill を撤去する
    Uninstall,
}
