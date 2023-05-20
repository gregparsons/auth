//! init.rs

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init(dot_env_path: &str) {

    match dotenvy::from_filename(dot_env_path) {
        Ok(_) => tracing::debug!(".env found"),
        _ => tracing::debug!(".env not found"),
    }
    tracing_subscriber::registry().with(fmt::layer()).with(EnvFilter::from_default_env()).init();

    tracing::debug!("[init] .env file: {}", dot_env_path);

}