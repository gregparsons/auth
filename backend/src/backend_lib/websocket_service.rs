//! websocket_service.rs
//!
//! There are several places where Alpaca documents the websocket API:
//! 1. https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/realtime/
//! 2. https://alpaca.markets/deprecated/docs/api-documentation/api-v2/streaming/
//! 3. https://alpaca.markets/docs/api-references/trading-api/streaming/
//! 4. https://alpaca.markets/docs/api-references/market-data-api/crypto-pricing-data/realtime/
//!


use std::time::{Duration};
/**
    Websocket client for Alpaca

    Current hard-coded stocks:
    aapl, tsla, plug, aal, nio, bac
*/
// use crossbeam_channel::{after, select, tick};
use crate::db::DbMsg;
use crossbeam::channel::Sender;
use serde_json::{json, Value};
use tungstenite::client::IntoClientRequest;
use tungstenite::{Message};
use common_lib::common_structs::{WsListenMessage, WsListenMessageData, MinuteBar, WsAuthenticate};
use common_lib::settings::Settings;
use crate::models::{AlpWsTrade};
// use crate::settings::{STOCK_LIST, STOCK_LIST_COUNT};

#[derive(PartialEq)]
pub enum AlpacaStream{
    TextData,
    BinaryUpdates,
}

// fn stock_list_to_uppercase(lower_stock:[&'static str; STOCK_LIST_COUNT])-> Vec<String>{
//     lower_stock.map(|x| x.to_uppercase() ).to_vec()
// }

fn stock_list_to_uppercase(lower_stock:&Vec<String>)-> Vec<String>{
    lower_stock.iter().map(|x| x.to_uppercase() ).collect()
}

pub struct Ws;

impl Ws {
    pub fn run(tx_db: Sender<DbMsg>, stream_type:&AlpacaStream, symbols:Vec<String>, settings:Settings) {
        tracing::debug!("[run]");
        Ws::ws_connect(tx_db, stream_type, symbols, &settings);
    }

    fn ws_connect(tx_db: Sender<DbMsg>, stream_type:&AlpacaStream, symbols:Vec<String>, settings:&Settings) {

        let ws_url = match stream_type{
            AlpacaStream::TextData => std::env::var("ALPACA_WS_URL_TEXT").expect("ALPACA_WS_URL_TEXT not found"),
            AlpacaStream::BinaryUpdates => std::env::var("ALPACA_WS_URL_BIN").expect("ALPACA_WS_URL_BIN not found"),
        };

        // websocket restart loop
        loop {

            let url = url::Url::parse(&ws_url).unwrap();
            let request = (&url).into_client_request().unwrap();

            // commence websocket connection
            match tungstenite::connect(request) {
                Err(e) => tracing::debug!("websocket connect error: {:?}", e),

                Ok((mut ws, _response)) => {
                    tracing::debug!("[ws_connect] successful websocket connection; response: {:?}", _response);

                    // todo: check if websocket connected; it won't if there's one already connected elsewhere; Alpaca sends an error
                    let auth_json = generate_ws_authentication_message(&settings);

                    // send authentication message
                    ws.write_message(Message::Text(auth_json)).unwrap();

                    loop {
                        // tracing::debug!("[ws_connect] reading websocket...");

                        // non-async version of tungstenite
                        if let Ok(msg) = ws.read_message() {

                            tracing::debug!("[ws_connect] read websocket...");

                            match msg {

                                Message::Ping(t) => {
                                    tracing::debug!("[ws_connect][ping] {:?}", &t)
                                    // spawn a thread to freak out if it doesn't hear a new ping...
                                    // todo: or just write to the database and ping it from another service
                                },
                                Message::Binary(b_msg) => {
                                    tracing::debug!("[ws_connect][binary] {:?}", &b_msg);
                                    match serde_json::from_slice::<Value>(&b_msg) {
                                        Ok(json) => {
                                            tracing::debug!("[ws_connect][binary] binary json: {:?}", &json);
                                            tracing::debug!("[ws_connect][binary] json[data][stream].as_str(): {:?}", json["stream"].as_str());
                                            if json["stream"].as_str().unwrap() == "authorization" {
                                                if json["data"]["action"].as_str().unwrap() == "authenticate" &&
                                                    json["data"]["status"].as_str().unwrap() == "authorized" {
                                                    tracing::debug!("[ws_connect][binary] authorized");

                                                    // SEND trade_updates request
                                                    let listen_msg = generate_ws_listen_message(vec!("trade_updates".to_string()));
                                                    tracing::debug!("[ws_connect][binary] outgoing listen msg: {}", &listen_msg);
                                                    let _ = ws.write_message(Message::Text(listen_msg));
                                                } else {
                                                    tracing::debug!("[ws_connect][binary] not authorized");
                                                }
                                            } else if json["stream"].as_str().unwrap() == "listening" {
                                                if let Ok(streams) = serde_json::from_value::<Vec<String>>(json["data"]["streams"].clone()) {
                                                    for stream in streams {
                                                        tracing::debug!("[ws_connect][binary] listening to stream: {}", &stream);
                                                    }
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            tracing::debug!("[ws_connect][binary] binary json conversion error: {:?}", &e);
                                        }
                                    }
                                }
                                Message::Text(t_msg) => {
                                    tracing::debug!("[ws_connect][text] {}",&t_msg);
                                    let json_vec: Vec<Value> = serde_json::from_str(&t_msg).unwrap();
                                    for json in json_vec {

                                        // https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/realtime/
                                        // [{"T":"success","msg":"connected"}]
                                        // https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/realtime/#minute-bar-schema

                                        // Parse "T"
                                        if let Some(alpaca_msg_type) = json["T"].as_str() {
                                            match alpaca_msg_type {
                                                "error" => {
                                                    if let Some(_msg) = &json["msg"].as_str() {
                                                        tracing::debug!("[ws_connect][text] msg: {}({})",&json["msg"].as_str().unwrap(),&json["code"].as_u64().unwrap());
                                                    }
                                                }
                                                "success" => {
                                                    // T:success messages "msg" can be "connected", "authenticated"
                                                    // [{"T":"success","msg":"connected"}]
                                                    // [{"T":"success","msg":"authenticated"}]

                                                    // Step 1, connect
                                                    if let Some(_msg) = &json["msg"].as_str() {
                                                        tracing::debug!("[ws_connect][text][success] msg: {:?}",&json["msg"].as_str().unwrap());
                                                        match json["msg"].as_str() {
                                                            Some(msg) => {

                                                                // Step 2, authenticate
                                                                if msg == "authenticated" {
                                                                    // subscribe to stock feeds
                                                                    // https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/realtime/#subscribe
                                                                    let json = json!({
                                                                        "action": "subscribe",
                                                                        "trades":  stock_list_to_uppercase(&symbols),
                                                                        // "quotes": STOCK_LIST_CAPS,
                                                                        "bars": stock_list_to_uppercase(&symbols),
                                                                    });
                                                                    tracing::debug!("[ws_connect] sending subscription request...\n{}", &json);
                                                                    let result = ws.write_message(Message::Text(json.to_string()));
                                                                    tracing::debug!("[ws_connect] subscription request sent: {:?}", &result);
                                                                }
                                                            }
                                                            None => {
                                                                tracing::debug!("[ws_connect][text][success] no message, needed 'authenticated'");
                                                            }
                                                        }
                                                    }
                                                }
                                                "subscription" => {
                                                    // subscription confirmation
                                                    // [{"T":"subscription","trades":["AAPL"],"quotes":["AMD","CLDR"],"bars":["IBM","AAPL","VOO"]}]
                                                    tracing::debug!("[ws_connect][subscription] {:?}",&json);

                                                    // subscription confirmed; get the latest; change the state machine to accepting updates
                                                    // (though not really necessary, can take them if they come)
                                                }
                                                "t" => {
                                                    // trade
                                                    match serde_json::from_value::<AlpWsTrade>(json) {
                                                        Ok(trade) => {
                                                            tracing::debug!("[ws_connect][text] trade: {:?}",&trade);
                                                            let _ = tx_db.send(DbMsg::WsTrade(trade.to_owned()));
                                                        },
                                                        Err(e) => {
                                                            tracing::debug!("[ws_connect][text] trade parse failed: {:?}", &e);
                                                        }
                                                    }
                                                },
                                                "b" => {
                                                    // minute bar schema
                                                    // https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/realtime/#minute-bar-schema
                                                    /*
                                                        {
                                                          "T": "b",
                                                          "S": "SPY",
                                                          "o": 388.985,
                                                          "h": 389.13,
                                                          "l": 388.975,
                                                          "c": 389.12,
                                                          "v": 49378,
                                                          "t": "2021-02-22T19:15:00Z"
                                                        }
                                                */
                                                    tracing::debug!("[ws_connect][text] minute bar: {:?}",&json);
                                                    let minute_bar_result = serde_json::from_value::<MinuteBar>(json);
                                                    match minute_bar_result {
                                                        Ok(minute_bar) => {
                                                            tracing::debug!("[ws_connect][text] minute_bar parsed: {:?}", &minute_bar);
                                                            let _ = tx_db.send(DbMsg::MinuteBar(minute_bar.to_owned()));
                                                        },
                                                        Err(e) => {
                                                            tracing::debug!("[ws_connect][text] minute_bar parse failed: {:?}", &e);
                                                        }
                                                    }
                                                },
                                                "q" | "d" | "s" => {
                                                    // quote, daily, status
                                                    tracing::debug!("[ws_connect][text][ quote, daily, status] {:?}", &json);
                                                }
                                                _ => {
                                                    // tracing::debug!("[ws_connect][other] {:?}", &t_msg);
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    tracing::debug!("[run] websocket isn't okay, got unrecognizable data: {:?}", &msg);
                                }
                            }
                        }
                    }
                }
            };

            // 5 second delay if the websocket goes down, then retry
            std::thread::sleep(Duration::from_millis(5000));

        }
    }
}



/// Generate the websocket message needed to authenticate/authorize.
///
/// https://alpaca.markets/docs/api-references/trading-api/streaming/
/// https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/realtime/
///
/// Authenticate using:
/// {"action": "auth", "key": "{KEY_ID}", "secret": "{SECRET}"}
///
/// Response:
/// [{"T":"success","msg":"authenticated"}]
///
///                 // authenticate example (old credentials)
///
///   $ wscat -c wss://stream.data.alpaca.markets/v2/iex
///     connected (press CTRL+C to quit)
/// {"action": "auth","key":"","secret":""}
///                    < {"stream":"authorization","data":{"action":"authenticate","status":"authorized"}}
///                    >  {"action": "listen", "data": {"streams": ["T.SPY"]}}
///                    < {"stream":"listening","data":{"streams":["T.SPY"]}}
///
fn generate_ws_authentication_message(settings:&Settings) -> String {
    // {"action": "authenticate","data": {"key_id": "???", "secret_key": "???"}}

    // TODO: add database setting "use_paper_or_live_key"
    let api_key = settings.alpaca_paper_id.clone();                     // std::env::var("ALPACA_API_ID").expect("ALPACA_API_ID");
    let api_secret = settings.alpaca_paper_secret.clone();              //std::env::var("ALPACA_API_SECRET").expect("ALPACA_API_SECRET");

    let json_obj = WsAuthenticate {
        action: "auth".to_owned(),
        key: api_key,
        secret: api_secret,
    };

    let j: serde_json::Value = serde_json::to_value(&json_obj).expect("[gen_subscribe_json] json serialize failed");
    j.to_string()
}

/// Generate the websocket message needed to request account and order status updates.
/// Return a string of formatted json.
/// https://alpaca.markets/docs/api-references/trading-api/streaming/#trade-updates
///
/// "Note: The trade_updates stream coming from wss://paper-api.alpaca.markets/stream uses binary
/// frames, which differs from the text frames that comes from the wss://data.alpaca.markets/stream stream."
/// (https://alpaca.markets/docs/api-references/trading-api/streaming/#streaming)
///
fn generate_ws_listen_message(streams_to_subscribe:Vec<String>) -> String{

    let listen_message = WsListenMessage {
        action: "listen".to_string(),
        data: WsListenMessageData {
            streams: streams_to_subscribe
        }
    };
    tracing::debug!("[gen_listen_json] listen_message: {:?}", &listen_message);
    serde_json::to_value(&listen_message).expect("[gen_listen_json] json serialize failed").to_string()

}
