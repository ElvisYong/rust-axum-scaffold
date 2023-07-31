pub mod health;

use axum::Router;

use crate::config::Config;

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let app = api_router();

    Ok(())
}

/// Top level router
fn api_router() -> Router {
    Router::new().nest("/", health::router())
}
