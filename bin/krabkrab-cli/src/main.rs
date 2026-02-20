use clap::{Parser, Subcommand};
use krabkrab::commands::{
    bridge_command, channels_add_command, channels_list_command, channels_logs_command,
    channels_remove_command, channels_status_command, config_edit_command, config_get_command,
    config_set_command, config_show_command, configure_command_interactive, cron_add_command,
    cron_list_command, daemon_command, devices_command, directory_command, discord_send_command,
    discord_send_dry_run_command, dns_command, docs_command, doctor_simple,
    exec_approvals_command, hooks_command, is_remote_environment, login_github_copilot,
    login_minimax_oauth, login_openai_codex_oauth_interactive, login_qwen_oauth, logs_tail_command,
    memory_search_command, memory_sync_command, models_auth_add_command, models_auth_get_command,
    models_auth_list_command, models_auth_remove_command, models_list_command, nodes_command,
    onboard_quick, onboard_wizard, pairing_approve_command, pairing_generate_command,
    pairing_list_command, run_interactive_shell, sandbox_command, send_whatsapp_media,
    send_whatsapp_message, skills_command, slack_send_command,
    slack_send_dry_run_command, status_simple, system_command, telegram_send_command,
    telegram_send_dry_run_command, update_command, webhooks_command,
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
        to: String,
        #[arg(long)]
        text: String,
        #[arg(long)]
        reply_to_message_id: Option<i64>,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
    Slack {
        #[arg(long)]
        to: String,
        #[arg(long)]
        text: String,
        #[arg(long)]
        thread_ts: Option<String>,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
    Discord {
        #[arg(long)]
        to: String,
        #[arg(long)]
        text: String,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
    WhatsApp {
        #[arg(long)]
        to: String,
        #[arg(long)]
        text: String,
        #[arg(long)]
        access_token: String,
        #[arg(long)]
        phone_number_id: String,
        #[arg(long)]
        media_url: Option<String>,
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
    ModelsAuth {
        #[command(subcommand)]
        sub: ModelsAuthSub,
    },
    Login {
        #[command(subcommand)]
        sub: LoginSub,
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
        channel: String,
        token: Option<String>,
    },
    Remove {
        channel: String,
    },
    Logs {
        channel: Option<String>,
        #[arg(long, short)]
        lines: Option<usize>,
    },
}

#[derive(Subcommand)]
enum ModelsAuthSub {
    List,
    Add {
        profile_id: String,
        provider: String,
        token: Option<String>,
    },
    Remove {
        profile_id: String,
    },
    Get {
        profile_id: String,
    },
}

#[derive(Subcommand)]
enum LoginSub {
    Minimax {
        #[arg(long, default_value = "global")]
        region: String,
    },
    Qwen,
    GithubCopilot {
        #[arg(long)]
        profile_id: Option<String>,
    },
    OpenAiCodex {
        #[arg(long)]
        profile_id: Option<String>,
        #[arg(long)]
        client_id: Option<String>,
        #[arg(long)]
        client_secret: Option<String>,
    },
}

#[derive(Subcommand)]
enum ConfigSub {
    Show,
    Get { key: String },
    Set { key: String, value: String },
    Edit,
}

#[derive(Subcommand)]
enum CronSub {
    List,
    Add { schedule: String, command: String },
    Remove { id: String },
    Enable { id: String },
    Disable { id: String },
}

#[derive(Subcommand)]
enum PairingSub {
    List,
    Generate,
    Approve { channel: String, code: String },
    Revoke { device_id: String },
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
        CliCommand::Status => println!("{}", status_simple()),
        CliCommand::Doctor => println!("{}", doctor_simple()),
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
        CliCommand::Shell {
            url,
            token,
            session,
        } => {
            use krabkrab::shell::ShellConfig;
            run_interactive_shell(ShellConfig {
                url,
                token,
                session,
            })
            .await?;
        }
        CliCommand::Telegram {
            to,
            text,
            reply_to_message_id,
            dry_run,
        } => {
            let out = if dry_run {
                telegram_send_dry_run_command(&to, &text, reply_to_message_id)?
            } else {
                telegram_send_command(&to, &text, reply_to_message_id).await?
            };
            println!("{out}");
        }
        CliCommand::Slack {
            to,
            text,
            thread_ts,
            dry_run,
        } => {
            let out = if dry_run {
                slack_send_dry_run_command(&to, &text, thread_ts.as_deref())?
            } else {
                slack_send_command(&to, &text, thread_ts.as_deref()).await?
            };
            println!("{out}");
        }
        CliCommand::Discord { to, text, dry_run } => {
            let out = if dry_run {
                discord_send_dry_run_command(&to, &text)?
            } else {
                discord_send_command(&to, &text).await?
            };
            println!("{out}");
        }
        CliCommand::WhatsApp {
            to,
            text,
            access_token,
            phone_number_id,
            media_url,
            dry_run,
        } => {
            if dry_run {
                let payload = if let Some(media_url) = media_url {
                    krabkrab::connectors::whatsapp_client::build_whatsapp_text_payload(&to, &text)
                } else {
                    krabkrab::connectors::whatsapp_client::build_whatsapp_text_payload(&to, &text)
                };
                println!(
                    "WhatsApp dry run: {}",
                    serde_json::to_string_pretty(&payload).unwrap()
                );
            } else {
                let result = if let Some(media_url) = media_url {
                    send_whatsapp_media(
                        &to,
                        Some(&text),
                        &media_url,
                        &access_token,
                        &phone_number_id,
                    )
                    .await?
                } else {
                    send_whatsapp_message(&to, &text, &access_token, &phone_number_id).await?
                };
                println!(
                    "WhatsApp message sent: {}",
                    serde_json::to_string_pretty(&result).unwrap()
                );
            }
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
        CliCommand::ModelsAuth { sub } => match sub {
            ModelsAuthSub::List => {
                println!("{}", models_auth_list_command());
            }
            ModelsAuthSub::Add {
                profile_id,
                provider,
                token,
            } => {
                let out = models_auth_add_command(&profile_id, &provider, token.as_deref())?;
                println!("{out}");
            }
            ModelsAuthSub::Remove { profile_id } => {
                let out = models_auth_remove_command(&profile_id)?;
                println!("{out}");
            }
            ModelsAuthSub::Get { profile_id } => {
                let out = models_auth_get_command(&profile_id)?;
                println!("{out}");
            }
        },
        CliCommand::Login { sub } => match sub {
            LoginSub::Minimax { region } => {
                let out = login_minimax_oauth(Some(&region))?;
                println!("access={} expires={}", out.access, out.expires);
                drop(out);
            }
            LoginSub::Qwen => {
                let out = login_qwen_oauth()?;
                println!("access={} expires={}", out.access, out.expires);
                drop(out); // ensure value is used
            }
            LoginSub::GithubCopilot { profile_id } => {
                let out = login_github_copilot(profile_id.as_deref())?;
                println!("{out}");
            }
            LoginSub::OpenAiCodex {
                profile_id,
                client_id,
                client_secret,
            } => {
                let is_remote = is_remote_environment();
                let out = login_openai_codex_oauth_interactive(
                    is_remote,
                    client_id.as_deref(),
                    client_secret.as_deref(),
                )?;
                println!("access_token={}", out.access_token);
                let _ = profile_id;
            }
        },
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
            ChannelsSub::Add { channel, token } => {
                println!("{}", channels_add_command(&channel, token.as_deref()))
            }
            ChannelsSub::Remove { channel } => println!("{}", channels_remove_command(&channel)),
            ChannelsSub::Logs { channel, lines } => {
                println!("{}", channels_logs_command(channel.as_deref(), lines))
            }
        },
        CliCommand::Logs {
            lines,
            follow,
            json,
        } => {
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
            CronSub::Add { schedule, command } => {
                println!("{}", cron_add_command(&schedule, &command))
            }
            CronSub::Remove { id } => println!("Removing cron job: {}", id),
            CronSub::Enable { id } => println!("Enabling cron job: {}", id),
            CronSub::Disable { id } => println!("Disabling cron job: {}", id),
        },
        CliCommand::Pairing { sub } => match sub {
            PairingSub::List => println!("{}", pairing_list_command()),
            PairingSub::Generate => println!("{}", pairing_generate_command()),
            PairingSub::Approve { channel, code } => {
                println!("{}", pairing_approve_command(&channel, &code))
            }
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
                    let mgr = browser::BrowserManager::new(&profile).await?;
                    let tabs = mgr.list_tabs().await?;
                    println!("{}", serde_json::to_string_pretty(&tabs)?);
                }
                BrowserSub::Open { profile, url } => {
                    let mgr = browser::BrowserManager::new(&profile).await?;
                    let tab = mgr.open_tab(&url).await?;
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
                    println!(
                        "upload ok profile={} selector={} files={}",
                        profile,
                        selector,
                        files.len()
                    );
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
            use krabkrab::tui::{run_tui, TuiConfig};
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
