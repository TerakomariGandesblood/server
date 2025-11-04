mod api;

use std::borrow::Cow;
use std::path::Path;
use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::{BoxError, Router, routing};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::decompression::RequestDecompressionLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tower_http::validate_request::ValidateRequestHeaderLayer;

const USERNAME: &str = "terakomari";

const PASSWORD: &str = "orange";

pub fn router(path: &Path) -> Router {
    Router::new()
        .route("/health-check", routing::get(api::health_check))
        .route(
            "/upload",
            routing::post(api::upload).layer(ValidateRequestHeaderLayer::basic(USERNAME, PASSWORD)),
        )
        .route(
            "/",
            routing::get(|| async { Redirect::permanent("/index.html") }),
        )
        .fallback_service(ServeDir::new(path))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new())
                .layer(TraceLayer::new_for_http())
                .layer(DefaultBodyLimit::max(128 * 1024 * 1024))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10 * 60)),
        )
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {error}")),
    )
}
