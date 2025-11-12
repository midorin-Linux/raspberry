mod app;
mod utils;
mod response;

use crate::app::config::Config;
use crate::utils::logging::init_tracing;
use std::net::Ipv4Addr;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("  Management Website (Version: alpha)");

    let config = Config::load()?;
    let _ = init_tracing(config.clone())?;

    println!("  - Local:    http://{}:{}", Ipv4Addr::LOCALHOST.to_string(), config.port.clone());

    println!("\n  Starting...");
    let mut app = app::runner::App::new(config)?;
    app.run().await?;

    Ok(())
}
