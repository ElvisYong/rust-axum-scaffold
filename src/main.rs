use std::sync::Arc;

use clap::Parser;
use rust_axum_scaffold::config::Config;

fn main() {
    // Initialize logger
    tracing_subscriber::fmt::init();

    // Initialize environment
    dotenv::dotenv().ok();
    let config = Arc::new(Config::parse());

}
