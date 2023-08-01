pub mod utils;
pub mod repositories;
pub mod services;
pub mod controllers;
pub mod config;

use std::sync::Arc;

use clap::Parser;
use config::AppConfig;

#[tokio::main]
async fn main() {
    // Initialize logger
    tracing_subscriber::fmt::init();

    // Initialize environment
    dotenv::dotenv().ok();
    let config = Arc::new(AppConfig::parse());

    controllers::server::serve(config).await.unwrap();
}
