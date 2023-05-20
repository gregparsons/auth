//! main.rs
#![forbid(unsafe_code)]

use backend_lib::data_collector::DataCollector;
use common_lib::init::init;
use common_lib::settings::Settings;
use common_lib::sqlx_pool::create_sqlx_pg_pool;

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

        let pool = create_sqlx_pg_pool().await;
        match Settings::load(&pool).await{
            Ok(settings) =>{
                DataCollector::start(pool, &settings).await;
            },
            Err(e) => {
                tracing::debug!("[main] could not load settings: {:?}", &e);
            }
        }
    });
}
