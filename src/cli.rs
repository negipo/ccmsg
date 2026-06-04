use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ccmsg", about = "Messaging between Claude Code sessions")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show unread messages addressed to you and mark them read (registers you as a peer)
    Inbox {
        #[arg(long)]
        project: String,
    },
    /// Block until a message arrives, then display it and mark it read
    Wait {
        #[arg(long)]
        project: String,
        #[arg(long, default_value = "60")]
        timeout: u64,
    },
    /// Send a message to a peer (from is the basename of project)
    Send {
        #[arg(long)]
        to: String,
        #[arg(long)]
        body: String,
        #[arg(long)]
        project: String,
    },
    /// List known peers (directory names)
    List,
    /// Reset all ccmsg state (messages and peers)
    Reset {
        /// Skip the confirmation prompt
        #[arg(long)]
        yes: bool,
    },
    /// Install skills to ~/.claude/skills/
    Install,
    /// Remove the installed skills
    Uninstall,
}
