use clap::{Parser, Subcommand};
use krabkrab_common::{info, init_logger};
use krabkrab_wizard::{run_onboarding_wizard, ConsolePrompter};

#[derive(Parser)]
#[command(name = "krabkrab", about = "KrabKrab CLI (Rust scaffold)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start gateway runtime (stub)
    Start,
    /// Send a message via agent runtime (stub)
    Send { to: String, message: String },
    /// Run onboarding wizard scaffold
    Onboard,
}

fn main() {
    init_logger();
    let cli = Cli::parse();

    match cli.command {
        Commands::Start => {
            info("Starting KrabKrab gateway (stub)");
            krabkrab_gateway::start_gateway();
        }
        Commands::Send { to, message } => {
            info("Sending message through KrabKrab agent (stub)");
            if let Err(err) = krabkrab_agents::send_message(&to, &message) {
                eprintln!("send failed: {err}");
                std::process::exit(1);
            }
        }
        Commands::Onboard => {
            info("Running KrabKrab onboarding wizard (stub)");
            let mut prompter = ConsolePrompter;
            match run_onboarding_wizard(&mut prompter) {
                Ok(result) => {
                    println!("Onboarding completed:");
                    println!("  flow: {:?}", result.flow);
                    println!("  profile: {}", result.profile_name);
                    println!("  gateway.host: {}", result.gateway.host);
                    println!("  gateway.port: {}", result.gateway.port);
                    println!("  gateway.auth_required: {}", result.gateway.auth_required);
                }
                Err(err) => {
                    eprintln!("onboarding failed: {err}");
                    std::process::exit(1);
                }
            }
        }
    }
}
