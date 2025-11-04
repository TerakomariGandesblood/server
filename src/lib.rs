mod args;
mod log;
mod router;
#[cfg(not(windows))]
mod service;

pub use args::*;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
pub use log::*;
pub use router::*;
#[cfg(not(windows))]
pub use service::*;
use tokio::signal;

pub struct ServerError(anyhow::Error);

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        tracing::error!("Something went wrong: {}", self.0);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl<E> From<E> for ServerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
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
