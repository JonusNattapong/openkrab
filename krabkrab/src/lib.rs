//! krabkrab — minimal Rust scaffold ported from `openclaw` (proof-of-concept)

pub mod commands;
pub mod daemon;
pub mod common;
pub mod config;
pub mod connectors;
pub mod channels;
pub mod gateway;
pub mod logging;
pub mod security;
pub mod utils;
pub mod version;

use once_cell::sync::Lazy;
use serde::Serialize;

pub static VERSION: Lazy<String> = Lazy::new(|| env!("CARGO_PKG_VERSION").to_string());

#[derive(Serialize)]
pub struct Hello {
    pub message: String,
    pub version: String,
}

pub fn hello() -> Hello {
    Hello { message: "hello from krabkrab".into(), version: VERSION.clone() }
}
