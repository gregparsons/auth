//! main.rs
#![forbid(unsafe_code)]

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};
use backend_lib::backend::Backend;


/// load the .env file and initialize logging
/// todo move to a "common" lib
fn init(dot_env_path: &str) {
    tracing::debug!(".env file: {}", dot_env_path);

    match dotenvy::from_filename(dot_env_path) {
        Ok(_) => tracing::debug!(".env found"),
        _ => tracing::debug!(".env not found"),
    }

    tracing_subscriber::registry().with(fmt::layer()).with(EnvFilter::from_default_env()).init();
}

/// main
fn main() {
    // this was useful in another cargo workspace where the path was specific to the crate inside the workspace
    init(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));

    // if you care to see what your macros are doing...
    let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .on_thread_start(|| {})
        .on_thread_stop(|| {})
        .thread_name("alpaca")
        .enable_all()
        .build()
        .expect("Tokio runtime didn't start");

    tokio_runtime.block_on(async {
        Backend::start().await;
    });
}
