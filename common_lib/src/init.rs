//! init.rs

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init(package_name:&str) {

    let config_location = std::env::var("CONFIG_LOCATION").unwrap_or_else(|_| "not_docker".to_owned());

    println!("[init] config_location: {}", &config_location);

    let dot_env_path = match config_location.as_str() {
        "docker" => {
            ".env".to_string()
        },
        "not_docker" | _ =>{
            // backend/.env
            // frontend/.env
            // etc
            format!("{}/.env", &package_name)
        }
    };

    let env_file_version = std::env::var("ENV_FILE_VERSION").unwrap_or_else(".env not loaded".to_string());

    println!("[init] dot_env_path: {}; env_file_version: {}", &dot_env_path, &env_file_version);

    match dotenvy::from_filename(&dot_env_path) {
        Ok(_) => println!("[init] .env found"),
        _ => println!("[init] .env not found"),
    }
    tracing_subscriber::registry().with(fmt::layer()).with(EnvFilter::from_default_env()).init();
    tracing::debug!("[init] .env file: {}", &dot_env_path);

}