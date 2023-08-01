use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use crate::services::service_register::ServiceRegister;

pub fn router() -> Router<Arc<ServiceRegister>> {
    Router::new()
        .route("/users", get(get_user))
}

async fn get_user() -> Response {
    (StatusCode::OK, "Hello World").into_response()
}
