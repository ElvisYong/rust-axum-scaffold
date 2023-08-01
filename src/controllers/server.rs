use std::sync::Arc;

use axum::Router;

use crate::{config::AppConfig, services::service_register::ServiceRegister};

use super::{health, user};

/// Server entry point
pub async fn serve(config: Arc<AppConfig>) -> anyhow::Result<()> {
    // Register Services
    let services = Arc::new(ServiceRegister::new(config).await);

    let app: Router<Arc<ServiceRegister>> = Router::new()
        .nest("/", health::router())
        .nest("/users", user::router())
        .with_state(services);

    Ok(())
}
