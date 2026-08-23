use std::{env, net::SocketAddr};

use axum::{routing::get, Json, Router};
use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio::net::TcpListener;
use tracing::info;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
    hostname: String,
    pod_name: Option<String>,
    pod_namespace: Option<String>,
    pod_ip: Option<String>,
    node_name: Option<String>,
    timestamp: DateTime<Utc>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "heartbeat_api=info,tower_http=info".into()),
        )
        .init();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{port}")
        .parse()
        .expect("PORT must be a valid TCP port");

    let app = Router::new().route("/health/", get(health));
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    info!(%addr, "starting heartbeat-api");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server failed");
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy",
        service: "heartbeat-api",
        version: env!("CARGO_PKG_VERSION"),
        hostname: hostname::get()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "unknown".to_string()),
        pod_name: env::var("POD_NAME")
            .ok()
            .or_else(|| env::var("HOSTNAME").ok()),
        pod_namespace: env::var("POD_NAMESPACE").ok(),
        pod_ip: env::var("POD_IP").ok(),
        node_name: env::var("NODE_NAME").ok(),
        timestamp: Utc::now(),
    })
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
