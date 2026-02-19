use clap::{Parser, Subcommand};
use krabkrab::commands::{
    configure_command_interactive, memory_search_command, memory_sync_command, slack_send_command,
    status_command, telegram_send_command, models_list_command, discord_send_command, discord_send_dry_run_command,
    doctor_command, onboard_wizard, onboard_quick, run_interactive_shell, bridge_command,
    channels_list_command, channels_status_command, channels_add_command, channels_remove_command,
    channels_logs_command,
    logs_tail_command, config_show_command, config_get_command, config_set_command, config_edit_command,
    cron_list_command, cron_add_command,
    pairing_list_command, pairing_approve_command, pairing_generate_command,
    update_command, skills_command, sandbox_command, nodes_command,
    hooks_command, webhooks_command, exec_approvals_command, docs_command, dns_command,
    directory_command, system_command, devices_command, daemon_command,
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
        #[command(subcommand)]
        sub: Option<OnboardSub>,
        #[arg(long, default_value = "default")]
        profile: String,
    },
    Shell {
        #[arg(long, default_value = "http://localhost:18789")]
        url: String,
        #[arg(long)]
        token: Option<String>,
        #[arg(long, default_value = "main")]
        session: String,
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
    Bridge {
        #[arg(long)]
        feature: String,
        #[arg(long)]
        action: Option<String>,
        #[arg(long)]
        payload: Option<String>,
        #[arg(long)]
        layer: Option<String>,
    },
    Channels {
        #[command(subcommand)]
        sub: ChannelsSub,
    },
    Logs {
        #[arg(long, short)]
        lines: Option<usize>,
        #[arg(long, short)]
        follow: bool,
        #[arg(long)]
        json: bool,
    },
    Config {
        #[command(subcommand)]
        sub: ConfigSub,
    },
    Cron {
        #[command(subcommand)]
        sub: CronSub,
    },
    Pairing {
        #[command(subcommand)]
        sub: PairingSub,
    },
    Update {
        #[arg(long, default_value_t = false)]
        channels: bool,
    },
    Skills {
        #[arg(long, default_value = "list")]
        action: String,
    },
    Sandbox {
        #[arg(long, default_value = "status")]
        action: String,
    },
    Nodes {
        #[arg(long, default_value = "list")]
        action: String,
    },
    Browser {
        #[command(subcommand)]
        sub: BrowserSub,
    },
    Hooks,
    Webhooks {
        #[arg(long, default_value = "list")]
        action: String,
    },
    #[command(name = "exec-approvals")]
    ExecApprovals {
        #[arg(long, default_value = "list")]
        action: String,
    },
    Docs {
        #[arg(long)]
        topic: Option<String>,
    },
    Dns {
        #[arg(long, default_value = "discover")]
        action: String,
    },
    Directory {
        #[arg(long, default_value = "list")]
        action: String,
    },
    System,
    Devices {
        #[arg(long, default_value = "list")]
        action: String,
    },
    Daemon {
        #[arg(long, default_value = "status")]
        action: String,
    },
    Tui {
        #[arg(long, default_value = "http://localhost:18789")]
        url: String,
        #[arg(long, default_value = "main")]
        session: String,
    },
}

#[derive(Subcommand)]
enum OnboardSub {
    /// Full interactive wizard (default)
    Wizard,
    /// Quick setup with minimal prompts
    Quick,
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

#[derive(Subcommand)]
enum ChannelsSub {
    List,
    Status,
    Add {
        #[arg(long)]
        channel: String,
        #[arg(long)]
        token: Option<String>,
    },
    Remove {
        #[arg(long)]
        channel: String,
    },
    Logs {
        #[arg(long)]
        channel: Option<String>,
        #[arg(long, short)]
        lines: Option<usize>,
    },
}

#[derive(Subcommand)]
enum ConfigSub {
    Show,
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Edit,
}

#[derive(Subcommand)]
enum CronSub {
    List,
    Add {
        schedule: String,
        command: String,
    },
    Remove {
        id: String,
    },
    Enable {
        id: String,
    },
    Disable {
        id: String,
    },
}

#[derive(Subcommand)]
enum PairingSub {
    List,
    Generate,
    Approve {
        channel: String,
        code: String,
    },
    Revoke {
        device_id: String,
    },
}

#[derive(Subcommand)]
enum BrowserSub {
    ProfileAdd {
        #[arg(long)]
        name: String,
        #[arg(long)]
        cdp_url: String,
    },
    ProfileRemove {
        #[arg(long)]
        name: String,
    },
    ProfileList,
    Tabs {
        #[arg(long, default_value = "default")]
        profile: String,
    },
    Open {
        #[arg(long, default_value = "default")]
        profile: String,
        #[arg(long)]
        url: String,
    },
    Navigate {
        #[arg(long, default_value = "default")]
        profile: String,
        #[arg(long)]
        url: String,
    },
    Click {
        #[arg(long, default_value = "default")]
        profile: String,
        #[arg(long)]
        selector: String,
    },
    Type {
        #[arg(long, default_value = "default")]
        profile: String,
        #[arg(long)]
        selector: String,
        #[arg(long)]
        text: String,
    },
    Upload {
        #[arg(long, default_value = "default")]
        profile: String,
        #[arg(long)]
        selector: String,
        #[arg(long = "file")]
        files: Vec<String>,
    },
    Snapshot {
        #[arg(long, default_value = "default")]
        profile: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.command.unwrap_or(CliCommand::Hello) {
        CliCommand::Hello => println!("{}", krabkrab::hello().message),
        CliCommand::Status => println!("{}", status_command()),
        CliCommand::Doctor => println!("{}", doctor_command()),
        CliCommand::Onboard { sub, profile } => {
            match sub {
                Some(OnboardSub::Wizard) | None => {
                    onboard_wizard()?;
                }
                Some(OnboardSub::Quick) => {
                    onboard_quick()?;
                }
            }
            let _ = profile; // profile is used internally by wizard
        }
        CliCommand::Shell { url, token, session } => {
            use krabkrab::shell::ShellConfig;
            run_interactive_shell(ShellConfig {
                url,
                token,
                session,
            }).await?;
        }
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
        CliCommand::Bridge {
            feature,
            action,
            payload,
            layer,
        } => {
            let out = bridge_command(
                &feature,
                action.as_deref(),
                payload.as_deref(),
                layer.as_deref(),
            )?;
            println!("{out}");
        }
        CliCommand::Channels { sub } => match sub {
            ChannelsSub::List => println!("{}", channels_list_command()),
            ChannelsSub::Status => println!("{}", channels_status_command()),
            ChannelsSub::Add { channel, token } => println!("{}", channels_add_command(&channel, token.as_deref())),
            ChannelsSub::Remove { channel } => println!("{}", channels_remove_command(&channel)),
            ChannelsSub::Logs { channel, lines } => println!("{}", channels_logs_command(channel.as_deref(), lines)),
        },
        CliCommand::Logs { lines, follow, json } => {
            println!("{}", logs_tail_command(lines, follow, json));
        }
        CliCommand::Config { sub } => match sub {
            ConfigSub::Show => println!("{}", config_show_command()),
            ConfigSub::Get { key } => println!("{}", config_get_command(&key)),
            ConfigSub::Set { key, value } => println!("{}", config_set_command(&key, &value)),
            ConfigSub::Edit => println!("{}", config_edit_command()),
        },
        CliCommand::Cron { sub } => match sub {
            CronSub::List => println!("{}", cron_list_command()),
            CronSub::Add { schedule, command } => println!("{}", cron_add_command(&schedule, &command)),
            CronSub::Remove { id } => println!("Removing cron job: {}", id),
            CronSub::Enable { id } => println!("Enabling cron job: {}", id),
            CronSub::Disable { id } => println!("Disabling cron job: {}", id),
        },
        CliCommand::Pairing { sub } => match sub {
            PairingSub::List => println!("{}", pairing_list_command()),
            PairingSub::Generate => println!("{}", pairing_generate_command()),
            PairingSub::Approve { channel, code } => println!("{}", pairing_approve_command(&channel, &code)),
            PairingSub::Revoke { device_id } => println!("Revoking device: {}", device_id),
        },
        CliCommand::Update { channels } => println!("{}", update_command(channels)),
        CliCommand::Skills { action } => println!("{}", skills_command(&action)),
        CliCommand::Sandbox { action } => println!("{}", sandbox_command(&action)),
        CliCommand::Nodes { action } => println!("{}", nodes_command(&action)),
        CliCommand::Browser { sub } => {
            use krabkrab::browser;
            match sub {
                BrowserSub::ProfileAdd { name, cdp_url } => {
                    browser::register_profile(&name, &cdp_url)?;
                    println!("browser profile added: {} -> {}", name, cdp_url);
                }
                BrowserSub::ProfileRemove { name } => {
                    if browser::remove_profile(&name) {
                        println!("browser profile removed: {}", name);
                    } else {
                        println!("browser profile not found: {}", name);
                    }
                }
                BrowserSub::ProfileList => {
                    let profiles = browser::list_profiles();
                    if profiles.is_empty() {
                        println!("no browser profiles configured");
                    } else {
                        for p in profiles {
                            println!("{} {}", p.name, p.cdp_http_url);
                        }
                    }
                }
                BrowserSub::Tabs { profile } => {
                    let tabs = browser::list_tabs(&profile).await?;
                    println!("{}", serde_json::to_string_pretty(&tabs)?);
                }
                BrowserSub::Open { profile, url } => {
                    let tab = browser::open_tab(&profile, &url).await?;
                    println!("{}", serde_json::to_string_pretty(&tab)?);
                }
                BrowserSub::Navigate { profile, url } => {
                    browser::navigate(&profile, &url).await?;
                    println!("navigate ok profile={} url={}", profile, url);
                }
                BrowserSub::Click { profile, selector } => {
                    browser::click(&profile, &selector).await?;
                    println!("click ok profile={} selector={}", profile, selector);
                }
                BrowserSub::Type {
                    profile,
                    selector,
                    text,
                } => {
                    browser::type_text(&profile, &selector, &text).await?;
                    println!("type ok profile={} selector={}", profile, selector);
                }
                BrowserSub::Upload {
                    profile,
                    selector,
                    files,
                } => {
                    browser::upload_files(&profile, &selector, &files).await?;
                    println!("upload ok profile={} selector={} files={}", profile, selector, files.len());
                }
                BrowserSub::Snapshot { profile } => {
                    let snap = browser::snapshot(&profile).await?;
                    println!("{}", serde_json::to_string_pretty(&snap)?);
                }
            }
        }
        CliCommand::Hooks => println!("{}", hooks_command()),
        CliCommand::Webhooks { action } => println!("{}", webhooks_command(&action)),
        CliCommand::ExecApprovals { action } => println!("{}", exec_approvals_command(&action)),
        CliCommand::Docs { topic } => println!("{}", docs_command(topic.as_deref())),
        CliCommand::Dns { action } => println!("{}", dns_command(&action)),
        CliCommand::Directory { action } => println!("{}", directory_command(&action)),
        CliCommand::System => println!("{}", system_command()),
        CliCommand::Devices { action } => println!("{}", devices_command(&action)),
        CliCommand::Daemon { action } => println!("{}", daemon_command(&action)),
        CliCommand::Tui { url, session } => {
            use krabkrab::tui::{TuiConfig, run_tui};
            let config = TuiConfig {
                gateway_url: url,
                session,
                theme: "dark".to_string(),
            };
            if let Err(e) = run_tui(config) {
                eprintln!("TUI error: {}", e);
            }
        }
    }
    Ok(())
}
