use axum::{Router, routing::get, http::StatusCode};

pub fn router() -> Router {
    Router::new().route("/health", get(get_health_check))
}

async fn get_health_check() -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}