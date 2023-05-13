//! backend

/*

    Market

    Starts both REST and websocket services.

    Starts a DB thread to store results of rest/ws tickers.

    Performs analysis on incoming tickers.


*/


use std::str::FromStr;
use crate::common::sqlx_pool::create_sqlx_pg_pool;
use crate::common::symbol_list::get_symbols;
use crate::websocket_service::AlpacaStream;

pub struct Backend {}

impl Backend {
    pub async fn start() {

        let pool = create_sqlx_pg_pool().await;

        // Postgres Database
        // Start the long-running database thread;
        // get a sender from the Database Service.
        // tx = "transmitter"
        let tx_db = crate::db::start().await;
        tracing::debug!("[Market::start] db start() complete");

        // Websocket (Incoming) Data Service
        let alpaca_ws_on = bool::from_str(std::env::var("ALPACA_WEBSOCKET_ON").unwrap_or_else(|_| "false".to_owned()).as_str()).unwrap_or(false);
        tracing::info!("ALPACA_WEBSOCKET_ON is: {}", &alpaca_ws_on);

        if alpaca_ws_on {
            // spawn long-running text thread
            let tx_db_ws = tx_db.clone();
            tracing::debug!("Starting text websocket service in new thread...");

            let ws_pool = pool.clone();
            match get_symbols(&ws_pool).await{
                Ok(symbols) => {
                    let _ = std::thread::spawn(move || {
                        crate::websocket_service::Ws::run(tx_db_ws, &AlpacaStream::TextData, symbols);
                    });
                },
                Err(e) => {
                    tracing::debug!("[start] error getting symbols for websocket: {:?}", &e);
                }
            }

            // spawn binary websocket
            // let tx_db_ws2 = tx_db.clone();
            // let tx_trader_ws2 = tx_trader.clone();
            // tracing::debug!("Starting text websocket service in new thread...");
            // let _ = std::thread::spawn(move || {
            //     crate::websocket_service::Ws::run(tx_db_ws2, tx_trader_ws2, &AlpacaStream::BinaryUpdates);
            // });

        // if the websocket thread dies, the program finishes.
        // thread_websocket.join();
        } else {
            tracing::debug!("Websocket service not started, ALPACA_WEBSOCKET_ON is false");
        }

        // Rest HTTP Service (in/out)
        let alpaca_rest_on = bool::from_str(
            std::env::var("ALPACA_REST_ON")
                .unwrap_or_else(|_| "false".to_owned())
                .as_str(),
        )
        .unwrap_or(false);
        tracing::info!("ALPACA_REST_ON is: {}", alpaca_rest_on);

        if alpaca_rest_on {
            tracing::debug!("[Market::start] starting alpaca web client");

            crate::web_client_service::run(/*stocks, tx_db, tx_trader*/).await;

            tracing::debug!("[Market::start] alpaca web client finished");
        } else {
            tracing::debug!("Rest service not started, ALPACA_REST_ON is false");
        }

        // infinite loop to keep child threads alive
        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    }
}
