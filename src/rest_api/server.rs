//! Axum HTTP server for the REST API

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::controller::ControllerState;
use crate::error::{Error, Result};

use super::handlers;

/// Run the REST API server
pub async fn run_server(state: Arc<ControllerState>) -> Result<()> {
    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/api/v1/nodes", get(handlers::list_nodes))
        .route("/api/v1/nodes/:namespace/:name", get(handlers::get_node))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("REST API server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| Error::ConfigError(format!("Failed to bind to {}: {}", addr, e)))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| Error::ConfigError(format!("Server error: {}", e)))?;

    Ok(())
}
