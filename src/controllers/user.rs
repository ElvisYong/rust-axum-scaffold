use axum::{
    extract::{Path, State},
    response::Response,
    routing::get,
    Router,
};

use crate::services::{service_register::ServiceRegister, user_service::UserService};

pub fn router() -> Router<ServiceRegister> {
    Router::new().route("/user/:id", get(get_current_user))
}

async fn get_current_user(
    Path(id): Path<String>,
    State(user_service): State<UserService>,
) -> Response {
    let current_user = user_service.get_current_user(id).await?;
    Ok(Json({}))
}
