use std::sync::Arc;

use anyhow::Context;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::cors::{self, CorsLayer};
use tracing::log::info;

use crate::{config::AppConfig, services::service_register::ServiceRegister, utils::openapi_generator};

use super::{health, user_controller};

/// Server entry point where we register the services and start the server
pub async fn serve(config: Arc<AppConfig>) -> anyhow::Result<()> {
    // First generate the openapi.json file
    openapi_generator::generate_openapi_json();

    // Register Services to be used in handlers
    let services = ServiceRegister::new(config.clone()).await;

    let app = Router::new()
        .nest("/", health::router())
        .nest("/", user_controller::router())
        .with_state(services) // Inject services into handlers as state
        .layer(
            // Use ServiceBuilder to apply multiple middleware
            // This will ensure that the middleware is applied in the order from top to bottom
            // Read https://docs.rs/axum/latest/axum/middleware/index.html#ordering for more info
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    // .allow_credentials(true)
                    // .allow_methods([
                    //     Method::GET,
                    //     Method::POST,
                    //     Method::OPTIONS,
                    //     Method::DELETE,
                    //     Method::PUT,
                    // ])
                    // .allow_headers([AUTHORIZATION, ACCEPT, COOKIE, CONTENT_TYPE]),
                    .allow_origin(cors::Any), // In a real application, you should validate the `Origin` header.
            ),
        );

    info!("Starting server at {}", config.server_address);
    axum::Server::bind(&config.server_address.parse()?)
        .serve(app.into_make_service())
        .await
        .context("Error starting server")
}
