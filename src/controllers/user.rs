use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use crate::services::{service_register::ServiceRegister, user_service::UserService};

pub fn router() -> Router<ServiceRegister> {
    Router::new().route("/users", get(get_user))
}

async fn get_user(State(user_service): State<UserService>) -> Response {
    (StatusCode::OK, "Hello World").into_response()
}
