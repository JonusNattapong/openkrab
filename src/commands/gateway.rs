use crate::gateway::{start_gateway, GatewayServerOptions};
use anyhow::Result;

pub async fn gateway_start_command(_db_path: Option<&str>) -> Result<()> {
    let _server = start_gateway(GatewayServerOptions::default()).await?;
    Ok(())
}
