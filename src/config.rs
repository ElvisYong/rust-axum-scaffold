use clap::Parser;

/// See .env.sample in the root for details
#[derive(Parser)]
pub struct AppConfig {

    // Aws related envs
    #[clap(env)]
    aws_access_key_id: String,
    #[clap(env)]
    aws_secret_access_key: String,
    #[clap(env)]
    aws_region: String,

    // Optional Envs
    // Defaulted to 10 retries if not specified
    #[clap(env)]
    pub aws_max_retries: Option<u32>
}
