//! krabkrab — incremental Rust port of `openkrab` (TypeScript → Rust).

pub mod acp;
pub mod agents;
pub mod auto_reply;
pub mod broadcast;
pub mod browser;
pub mod canvas_host;
pub mod channels;
pub mod commands;
pub mod common;
pub mod compat;
pub mod config;
pub mod config_io;
pub mod config_validation;
pub mod connectors;
pub mod cron;
pub mod daemon;
pub mod dashboard;
pub mod diagnostics;
pub mod gateway;
pub mod hooks;
pub mod infra;
pub mod link_understanding;
pub mod llm_task;
pub mod logging;
mod logging_impl;
pub mod markdown;
pub mod matrix;
pub mod media;
pub mod media_understanding;
pub mod memory;
pub mod node_host;
pub mod oauth;
pub mod openkrab_config;
pub mod pairing;
pub mod plugin_sdk;
pub mod plugins;
pub mod polls;
pub mod process;
pub mod providers;
pub mod routing;
pub mod security;
pub mod secure;
pub mod security_audit;
pub mod shared;
pub mod signature;
pub mod skills;
pub mod sessions;
pub mod shell;
pub mod signal;
pub mod terminal;
pub mod thread_ownership;
pub mod tools;
pub mod tts;
pub mod tui;
pub mod utils;
pub mod version;
pub mod voice;
pub mod web_connector;
pub mod webrtc;
pub mod whatsapp;
pub mod wizard;

use once_cell::sync::Lazy;
use serde::Serialize;

pub static VERSION: Lazy<String> = Lazy::new(|| env!("CARGO_PKG_VERSION").to_string());

#[derive(Serialize)]
pub struct Hello {
    pub message: String,
    pub version: String,
}

pub fn hello() -> Hello {
    Hello {
        message: "hello from krabkrab".into(),
        version: VERSION.clone(),
    }
}
