use std::sync::Arc;

use anyhow::Context;
use axum::Router;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{self, CorsLayer};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    config::AppConfig,
    controllers::{health, user_controller},
    services::service_register::ServiceRegister,
    utils::openapi_generator,
};

/// Server entry point where we register the services and start the server
pub async fn serve(config: Arc<AppConfig>) -> anyhow::Result<()> {
    // First generate the openapi.json file
    let openapi = openapi_generator::generate_openapi_json(
        config
            .openapi_server_address
            .clone()
            .unwrap_or(config.server_address.clone()),
    );

    // Register Services to be used in handlers
    let services = ServiceRegister::new(config.clone()).await;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .nest("/", health::router())
        .nest("/", user_controller::router())
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
        ).with_state(services); // Inject services into handlers as state

    tracing::info!("Listening on port 3000");
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .context("Failed to start server")
}
