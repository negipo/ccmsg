mod cli;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Inbox { project } => ccmsg::commands::inbox::run(&project)?,
        Commands::Wait { project, timeout } => ccmsg::commands::wait::run(&project, timeout)?,
        Commands::Send { to, body, project } => ccmsg::commands::send::run(&to, &body, &project)?,
        Commands::List => ccmsg::commands::list::run()?,
        Commands::Reset { yes } => ccmsg::commands::reset::run(yes)?,
        Commands::Install => ccmsg::commands::install::run()?,
        Commands::Uninstall => ccmsg::commands::uninstall::run()?,
        Commands::HookSessionStart => ccmsg::commands::hook_session_start::run()?,
    }
    Ok(())
}
