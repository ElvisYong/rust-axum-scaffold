use axum::{Router, routing::get, http::StatusCode};

use crate::services::service_register::ServiceRegister;

pub fn router() -> Router<ServiceRegister> {
    Router::new().route("/health", get(get_health_check))
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check passed"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "health",
)]
async fn get_health_check() -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}