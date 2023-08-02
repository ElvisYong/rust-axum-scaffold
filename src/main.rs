pub mod config;
pub mod controllers;
pub mod domain;
pub mod repositories;
pub mod services;
pub mod utils;
pub mod errors;

use std::sync::Arc;

use clap::Parser;
use config::AppConfig;

#[tokio::main]
async fn main() {
    // Initialize logger
    tracing_subscriber::fmt::init();

    // Initialize environment
    get_app_config();
    dotenv::dotenv().ok();
    let app_config = Arc::new(AppConfig::parse());

    // Start the server
    controllers::server::serve(app_config).await.unwrap();
}

// Seperating this so we can reuse it in tests
pub fn get_app_config() -> Arc<AppConfig> {
    dotenv::dotenv().ok();
    let app_config = Arc::new(AppConfig::parse());
    app_config
}