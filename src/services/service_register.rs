use std::sync::Arc;

use aws_config::{meta::region::RegionProviderChain, retry::RetryConfigBuilder, SdkConfig};

use crate::{config::AppConfig, repositories::user_repository::UserRepository};

use super::user_service::UserService;

// We will be implementing a substate for each router therefore we need to implement FromRef
// See https://docs.rs/axum/latest/axum/extract/struct.State.html#substates
// Check UserService for manual implementation of FromRef, alternatively you can use #[derive(FromRef)]
#[derive(Clone)]
pub struct ServiceRegister {
    // Reason for having Option is so that when we test we do not have to instantiate all services
    // If we do not have Option, we will have to instantiate all services even if we do not use it for that test
    // Alternatively we can use Extensions instead of State if we want to have router specific services
    // See https://docs.rs/axum/latest/axum/#sharing-state-with-handlers
    // In this case we are using State for compile time type safety
    pub user_service: Option<UserService>,
}

// Common place to instantiate all our services
impl ServiceRegister {
    pub async fn new(app_config: Arc<AppConfig>) -> Self {
        // Setup AWS Related Config
        let shared_config = get_aws_shared_config(app_config).await;

        // Setup UserService
        let user_repository = UserRepository::new(&shared_config, None).await;
        let user_service = UserService::new(user_repository);

        Self {
            user_service: Some(user_service),
        }
    }
}

/// Helper to get AWS Shared Config
pub async fn get_aws_shared_config(app_config: Arc<AppConfig>) -> SdkConfig {
    // Setup AWS Related Config
    let region_provider: RegionProviderChain =
        RegionProviderChain::default_provider().or_else("ap-southeast-1");
    let retry_config = RetryConfigBuilder::new()
        .max_attempts(app_config.aws_max_retries.unwrap_or(10))
        .build();

    aws_config::from_env()
        .region(region_provider)
        .retry_config(retry_config)
        .load()
        .await
}
