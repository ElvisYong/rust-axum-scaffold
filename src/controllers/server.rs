use std::{any::Any, sync::Arc};

use anyhow::Context;
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, COOKIE},
        HeaderValue, Method,
    },
    Router,
};
use tower::ServiceBuilder;
use tower_http::cors::{self, CorsLayer};

use crate::{config::AppConfig, services::service_register::ServiceRegister};

use super::{health, user};

/// Server entry point
pub async fn serve(config: Arc<AppConfig>) -> anyhow::Result<()> {
    // Register Services
    let services = Arc::new(ServiceRegister::new(config.clone()).await);

    let app = Router::new()
        .nest("/", health::router())
        .nest("/users", user::router())
        .with_state(services)
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::OPTIONS,
                    Method::DELETE,
                    Method::PUT,
                ])
                .allow_origin(cors::Any) // In a real application, you should validate the `Origin` header.
                .allow_headers([AUTHORIZATION, ACCEPT, COOKIE, CONTENT_TYPE]),
        );

    axum::Server::bind(&config.server_address.parse()?)
        .serve(app.into_make_service())
        .await
        .context("Error starting server")
}
