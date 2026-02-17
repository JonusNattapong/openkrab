use crate::auth_store::logout_web;
use anyhow::Result;
use std::path::Path;

pub fn logout_web_session(auth_dir: Option<&Path>) -> Result<bool> {
    logout_web(auth_dir)
}
