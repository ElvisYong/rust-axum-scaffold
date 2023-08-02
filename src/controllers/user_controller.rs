// Controller layer should simply act as the gateway
// Input level validations should be done in the controller layer or by a middleware/extractor
// e.g of input level validations are: Checking if a username should not be more than 20 characters and it should be unique

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{
    domain::user::view_models::UserViewModel,
    errors::AppResult,
    services::{service_register::ServiceRegister, user_service::UserService},
};

pub fn router() -> Router<ServiceRegister> {
    Router::new().route("/user/:id", get(get_current_user))
}

#[utoipa::path(
    get,
    path = "/user/:id",
    responses(
        (status = 200, description = "Successfully retrieved user", body = [UserViewModel]),
        (status = 500, description = "Internal Server Error", body = [AppError]),
    ),
    tag = "user",
)]
pub async fn get_current_user(
    Path(id): Path<String>,
    State(user_service): State<UserService>,
) -> AppResult<Json<UserViewModel>> {
    let current_user = user_service.get_current_user(id).await?;

    Ok(Json(current_user))
}
