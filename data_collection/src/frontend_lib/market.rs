//! market.rs

/*

    Market

    Starts both REST and websocket services.

    Starts a DB thread to store results of rest/ws tickers.

    Performs analysis on incoming tickers.


*/

use crate::configuration::get_yaml_configuration;
use crate::frontend::routes;
use std::str::FromStr;
use crate::websocket_service::AlpacaStream;

pub struct Market {}

impl Market {
    pub async fn start() {

        // TODO: placeholder for future config file capability
        let settings = get_yaml_configuration().expect("no configuration.yaml");
        let address = format!("127.0.0.1:{}", settings.database.port);
        tracing::debug!("[main] address from config: {}", &address);

        // start the web server
        routes::run(&settings).await;




        // Postgres Database
        // Start the long-running database thread;
        // get a sender from the Database Service.
        // tx = "transmitter"
        let tx_db = crate::db::start().await;
        tracing::debug!("[Market::start] db start() complete");

        // Trader (Analysis) Service
        let tx_trader = crate::trader::start();
        tracing::info!("[Market::start] frontend_ui start result: {:?}", tx_trader);

        // Websocket (Incoming) Data Service
        let alpaca_ws_on = bool::from_str(std::env::var("ALPACA_WEBSOCKET_ON").unwrap_or_else(|_| "false".to_owned()).as_str()).unwrap_or(false);
        tracing::info!("ALPACA_WEBSOCKET_ON is: {}", &alpaca_ws_on);

        if alpaca_ws_on {
            // spawn long-running text thread
            let tx_db_ws = tx_db.clone();
            let tx_trader_ws = tx_trader.clone();
            tracing::debug!("Starting text websocket service in new thread...");
            let _ = std::thread::spawn(move || {
                crate::websocket_service::Ws::run(tx_db_ws, tx_trader_ws, &AlpacaStream::TextData);
            });


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

            // let stocks = STOCK_LIST
            //     .map(|stock| stock.to_string())
            //     .to_vec();

            crate::web_client_service::run(/*stocks, tx_db, tx_trader*/).await;

            tracing::debug!("[Market::start] alpaca web client finished");
        } else {
            tracing::debug!("Rest service not started, ALPACA_REST_ON is false");
        }

        // infinite loop to keep child threads alive
        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));

            // TODO: perhaps check if the threads are alive and restart them; manage the other parts basically
            // TODO: check environment variables for signals from the outside to do things like start/stop websocket, start/stop trading
        }
    }
}
