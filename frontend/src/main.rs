//! main.rs
#![forbid(unsafe_code)]

use common_lib::init::init;
use frontend::frontend::web_server::WebServer;

/// main
fn main() {
    // this was useful in another cargo workspace where the path was specific to the crate inside the workspace
    // init(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));
    init(env!("CARGO_MANIFEST_DIR"));

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
        WebServer::run().await;
    });
}
