use clap::{Parser, Subcommand};

use krabkrab::commands::configure::ConfigureInput;
use krabkrab::commands::{configure_command, slack_send_command, status_command, telegram_send_command};

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
    Telegram {
        #[arg(long)]
        text: String,
    },
    Slack {
        #[arg(long)]
        text: String,
    },
    Configure {
        #[arg(long, default_value = "default")]
        profile: String,
        #[arg(long, default_value_t = false)]
        verbose: bool,
    },
}

fn main() {
    let opts = Opts::parse();
    match opts.command.unwrap_or(CliCommand::Hello) {
        CliCommand::Hello => println!("{}", krabkrab::hello().message),
        CliCommand::Status => println!("{}", status_command()),
        CliCommand::Telegram { text } => println!("{}", telegram_send_command(&text)),
        CliCommand::Slack { text } => println!("{}", slack_send_command(&text)),
        CliCommand::Configure { profile, verbose } => {
            let out = configure_command(ConfigureInput { profile, verbose });
            println!("{out}");
        }
    }
}
