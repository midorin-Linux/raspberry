use crate::app::{config::Config, shutdown::shutdown_signal};
use crate::response::index;
use std::{net::{Ipv4Addr, SocketAddr}, time::Duration};

use anyhow::{Context, Result};
use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    middleware as axum_middleware,
    response::{Html, IntoResponse, Json, Response},
    routing::{get, post, put, delete},
    Router,
};
use tokio::net::TcpListener;
use tracing::{debug, info, warn, error, instrument};
use tower_http::{
    services::ServeDir,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

pub struct App {
    port: u16
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self {
            port: config.port
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let app = Router::new()
            .layer((
                TraceLayer::new_for_http(),
                TimeoutLayer::new(Duration::from_secs(10))
            ))
            .route("/", get(index::root))
            .nest_service("/static", ServeDir::new("./static"));

        println!("  Ready!\n");
        let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, self.port.clone()))).await?;
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .context("  failed to start server")?;

        Ok(())
    }
}
