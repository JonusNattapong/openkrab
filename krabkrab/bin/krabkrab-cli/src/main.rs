use clap::{Parser, Subcommand};
use krabkrab::commands::{
    configure_command_interactive, memory_search_command, memory_sync_command, slack_send_command,
    status_command, telegram_send_command, models_list_command, discord_send_command, discord_send_dry_run_command,
    doctor_command, onboard_command,
};

#[derive(Parser)]
#[command(author, version, about)]
struct Opts {
    #[command(subcommand)]
    command: Option<CliCommand>,
}

#[derive(Subcommand)]
enum CliCommand {
    Hello,
    Status,
    Doctor,
    Onboard {
        #[arg(long, default_value = "default")]
        profile: String,
    },
    Telegram {
        #[arg(long)]
        text: String,
    },
    Slack {
        #[arg(long)]
        text: String,
    },
    Discord {
        #[arg(long)]
        to: String,
        #[arg(long)]
        text: String,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
    Configure {
        #[arg(long, default_value = "default")]
        profile: String,
        #[arg(long, default_value_t = false)]
        verbose: bool,
    },
    Ask {
        query: String,
        #[arg(long)]
        db: Option<String>,
    },
    Memory {
        #[command(subcommand)]
        sub: MemorySub,
    },
    Gateway {
        #[command(subcommand)]
        sub: GatewaySub,
    },
    Models {
        #[arg(long)]
        provider: String,
    },
}

#[derive(Subcommand)]
enum MemorySub {
    Sync {
        #[arg(long, default_value = "./")]
        path: String,
        #[arg(long)]
        db: Option<String>,
        #[arg(short, long, default_value_t = false)]
        watch: bool,
    },
    Search {
        query: String,
        #[arg(long)]
        db: Option<String>,
    },
}

#[derive(Subcommand)]
enum GatewaySub {
    Start {
        #[arg(long)]
        db: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.command.unwrap_or(CliCommand::Hello) {
        CliCommand::Hello => println!("{}", krabkrab::hello().message),
        CliCommand::Status => println!("{}", status_command()),
        CliCommand::Doctor => println!("{}", doctor_command()),
        CliCommand::Onboard { profile } => println!("{}", onboard_command(&profile)),
        CliCommand::Telegram { text } => println!("{}", telegram_send_command(&text)),
        CliCommand::Slack { text } => println!("{}", slack_send_command(&text)),
        CliCommand::Discord { to, text, dry_run } => {
            let out = if dry_run {
                discord_send_dry_run_command(&to, &text)?
            } else {
                discord_send_command(&to, &text).await?
            };
            println!("{out}");
        }
        CliCommand::Configure { .. } => {
            let out = configure_command_interactive();
            println!("{out}");
        }
        CliCommand::Ask { query, db } => {
            let out = krabkrab::commands::ask_command(&query, db.as_deref()).await?;
            println!("{out}");
        }
        CliCommand::Memory { sub } => match sub {
            MemorySub::Sync { path, db, watch } => {
                let out = memory_sync_command(&path, db.as_deref(), watch).await?;
                println!("{out}");
                if watch {
                    // Keep the task running
                    loop {
                        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
                    }
                }
            }
            MemorySub::Search { query, db } => {
                let out = memory_search_command(&query, db.as_deref()).await?;
                println!("{out}");
            }
        },
        CliCommand::Gateway { sub } => match sub {
            GatewaySub::Start { db } => {
                krabkrab::commands::gateway_start_command(db.as_deref()).await?;
            }
        },
        CliCommand::Models { provider } => {
            let out = models_list_command(&provider)?;
            println!("{out}");
        }
    }
    Ok(())
}
