// This is where we define our custom app errors and map them to http status codes
// You can rename AppResult into your own naming such as MyCoolApiResult

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub type AppResult<T> = Result<T, AppError>;

/// Having a custom error type will allow us to handle errors in a more structured way
/// This error enum will be used heavily by the controller and service layer to return errors
/// Error annotations using thiserror will allow us write custom error messages
/// The #[from] annotation will allow us to convert other types of error such as anyhow::Error into AppError
/// Introduce your own error types here whenever you need to
#[derive(thiserror::Error, Debug, ToSchema)]
pub enum AppError {
    #[schema(example = "Authentication is required to access this resource")]
    #[error("Authentication is required to access this resource")]
    Unauthorized,
    #[schema(example = "User is not authorized to access this resource")]
    #[error("User is not authorized to access this resource")]
    Forbidden,
    #[schema(example = "Bad request")]
    #[error("{0}")]
    BadRequest(String),
    #[schema(example = "Unprocessable entity request")]
    #[error("Unprocessable entity request")]
    UnprocessableEntity,
    #[schema(example = "Not found")]
    #[error("{0}")]
    NotFound(String),
    #[schema(example = "Object conflict")]
    #[error("{0}")]
    ObjectConflict(String),
    #[schema(example = "Unexpected error occurred")]
    #[error("Unexpected error occurred")]
    InternalServerError,
    #[schema(example = "Unexpected error occurred")]
    #[error("{0}")]
    InternalServerErrorWithMessage(String),
    #[schema(example = "Unexpected error occurred")]
    #[error(transparent)]
    SerdeDynamoError(#[from] serde_dynamo::Error),
    #[schema(example = "Unexpected error occurred")]
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

/// This implementation will allow us to convert our AppError into an Axum response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_message, status_text) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, AppError::Unauthorized.to_string(), "Unauthorized"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, AppError::Forbidden.to_string(), "Forbidden"),
            AppError::BadRequest(err) => (StatusCode::BAD_REQUEST, err, "Bad Request"),
            AppError::UnprocessableEntity => (StatusCode::UNPROCESSABLE_ENTITY, AppError::UnprocessableEntity.to_string(), "Unprocessable Entity"),
            AppError::NotFound(err) => (StatusCode::NOT_FOUND, err, "Not Found"),
            AppError::ObjectConflict(err) => (StatusCode::CONFLICT, err, "Conflict"),
            AppError::InternalServerErrorWithMessage(err) => (StatusCode::INTERNAL_SERVER_ERROR, err, "Internal Server Error"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string(), "Internal Server Error"),
        };

        let body = Json(ApiError::new(status_code.as_u16(), &error_message, status_text));

        (status_code, body).into_response()
    }
}


/// This is where we have our API error response struct
/// Feel free to design your own API Error response
/// For this example I am referencing Google's JSON API error response
/// https://cloud.google.com/apis/design/errors
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
    pub status: String,
}

impl ApiError {
    pub fn new(code: u16, message: &str, status: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            status: status.to_string(),
        }
    }
}

