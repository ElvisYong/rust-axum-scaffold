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

// Utoipa provides a macro to generate the openapi documentation for the handler
// The path macro should be used to annotate the handler that contains
// method, path, responses, and tag
// Using rust's /// comments, we can add path level description into the openapi documentation

/// Get current user
/// This endpoint will return the current user
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

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use tower::ServiceExt;

    use crate::{
        config::AppConfig,
        controllers::user_controller,
        get_app_config,
        repositories::user_repository::UserRepository,
        services::{
            service_register::{get_aws_shared_config, ServiceRegister},
            user_service::UserService,
        },
    };

    // We could use a mock database for testing, however for this example we will use the actual database
    // Feel free to use a mock database if you want
    // Using a real database here would make this an integration test
    async fn get_service_register(app_config: Arc<AppConfig>) -> ServiceRegister {
        let shared_config = get_aws_shared_config(app_config).await;
        let user_repository = UserRepository::new(&shared_config, None).await;
        let user_service = UserService::new(user_repository);

        ServiceRegister {
            user_service: Some(user_service),
        }
    }

    // Test success path
    #[tokio::test]
    async fn get_current_user_success() {
        // Arrange
        let app_config = get_app_config();
        let service_register = get_service_register(app_config).await;
        let router = user_controller::router().with_state(service_register);
        let request = Request::builder()
            .uri("/user/ppId123")
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();

        // Act
        let response = router.oneshot(request).await.unwrap();
        let status = response.status();

        // Assert
        assert_eq!(status, 200);
    }
}
