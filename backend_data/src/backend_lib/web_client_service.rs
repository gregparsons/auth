//! web_client_service.rs
//!
//! Restful Alpaca Poller

use chrono::{Utc};
use crate::alpaca_activity::get_activity_api;
use crate::common::{MARKET_EARLY_OPEN_TIME, MARKET_LATE_CLOSE_TIME};


use sqlx::{PgPool, Pool, Postgres};
use tokio::runtime::Handle;

/// start up the database pool
pub async fn create_sqlx_pg_pool() -> Pool<Postgres> {
    // DB connect; get some data; currently just one stock for the last three days
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in environment");
    let pool = PgPool::connect(&db_url).await.expect("[main] failed to connect to postgres");
    pool
}


/// Spawn a new thread to poll the Alpaca REST API
pub async fn run(/*stocks: Vec<String>, tx_database: Sender<DbMsg>, tx_trader: Sender<TraderMsg>*/) {

    // run an async runtime inside the thread; it's a mess to try to run code copied from elsewhere
    // that normally runs async but is now running in a thread; much easier to just start a new
    // tokio runtime than to try to deal with FnOnce etc
    // people asking why you'd want to do this: https://stackoverflow.com/questions/61292425/how-to-run-an-asynchronous-task-from-a-non-main-thread-in-tokio/63434522#63434522

    let tokio_handle = Handle::current();

    let pool = create_sqlx_pg_pool().await;

    std::thread::spawn(move || {


        // let mut pool_opt:Option<Pool<Postgres>> = None;
        //
        // tokio_handle.spawn( async move {
        //     pool_opt = Some(create_sqlx_pg_pool().await);
        // });
        //
        // // unwrap is okay since the only source of None is here.
        // let pool2 = pool_opt.take().unwrap();


        tracing::debug!("[run]");
        // let pool:Pool<Postgres> = get_pool().await;

        // let alpaca_url = std::env::var("ALPACA_API_URL").expect("ALPACA_API_URL");
        // let alpaca_id = std::env::var("ALPACA_API_ID").expect("ALPACA_API_ID");
        // let alpaca_secret = std::env::var("ALPACA_API_SECRET").expect("ALPACA_API_SECRET");
        let alpaca_poll_rate_ms: u64 = std::env::var("API_INTERVAL_MILLIS").unwrap_or_else(|_| "5000".to_string()).parse().unwrap_or(5000);
        let time_open_ny = MARKET_EARLY_OPEN_TIME.clone();
        let time_close_ny = MARKET_LATE_CLOSE_TIME.clone();


        loop {

            let pool3 = pool.clone();

            // Call the API if the market is open in NYC
            let time_current_ny = Utc::now().with_timezone(&chrono_tz::America::New_York).time();
            if time_current_ny >= time_open_ny && time_current_ny <= time_close_ny {
                tracing::debug!("[rest_service:start] NY time: {:?}, open: {:?}, close: {:?}", &time_current_ny, &time_open_ny, &time_close_ny);
                // Don't need this. Using websocket exclusively.
                // for stock in stocks.iter() {
                //     tracing::debug!("[rest_service:start] Market is open (on business days). NY time: {:?}open: {:?}, close: {:?}", &time_current_ny, &time_open_ny, &time_close_ny);
                //     let _ = crate::web_client_service::get_last_trade_rest(tx_database.clone(), stock, &alpaca_url, &alpaca_id, &alpaca_secret, tx_trader.clone());
                // }

                // Poll the activity API
                // https://stackoverflow.com/questions/61292425/how-to-run-an-asynchronous-task-from-a-non-main-thread-in-tokio/63434522#63434522
                tokio_handle.spawn( async move {
                    let _ = get_activity_api(pool3).await;
                });

            } else {
                tracing::debug!("[rest_service:start] market is closed. NY time: {:?}, open: {:?}, close: {:?}", &time_current_ny, &time_open_ny, &time_close_ny);
            }

            std::thread::sleep(std::time::Duration::from_millis(alpaca_poll_rate_ms));

        }
    });
}


//
// Call Alpaca API to get the most recent event for a single stock
//
// fn get_last_trade_rest(
//     tx_database: Sender<DbMsg>,
//     stock_symbol: &str,
//     data_url: &str,
//     api_id: &str,
//     api_secret: &str,
//     tx_trader: Sender<TraderMsg>,
// ) -> std::io::Result<()> {
//     // https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/historical/#latest-trade
//     // GET/v2/stocks/{symbol}/trades/latest
//
//     let url = format!("{}/v2/stocks/{}/trades/latest", data_url, stock_symbol);
//
//     let req = ureq::get(&url)
//         .set("APCA-API-KEY-ID", api_id)
//         .set("APCA-API-SECRET-KEY", api_secret)
//         .set("Accept", "application/json");
//
//     let result_resp: Result<Response, Error> = req.call();
//
//     tracing::debug!("[get_last_trade_rest] http response: {:?}", result_resp);
//
//     match result_resp {
//         Ok(resp) => {
//             if let Ok(json_string) = &resp.into_string() {
//                 tracing::debug!("[get_last_trade_rest] json_string: {:?}", &json_string);
//
//                 if let Ok(alpaca_rest_trade) = serde_json::from_str::<AlpacaTradeRest>(json_string)
//                 {
//                     tracing::debug!(
//                         "[get_last_trade_rest] json_object: {:?}",
//                         &alpaca_rest_trade
//                     );
//                     tracing::debug!(
//                         "[get_ticker] sending DbMsg::LastTrade: \n***** {:?}",
//                         &alpaca_rest_trade
//                     );
//
//                     // Make notifications
//                     // let _ = tx_trader.send(TraderMsg::Hello);
//                     let _ = tx_trader.send(TraderMsg::TradeRest(alpaca_rest_trade.clone()));
//                     let _ =
//                         tx_database.send(DbMsg::LastTrade(alpaca_rest_trade /*.to_owned()*/));
//                 }
//             }
//         }
//         Err(e) => {
//             tracing::debug!("[web_service] error: {}", e)
//         }
//     }
//     Ok(())
// }
