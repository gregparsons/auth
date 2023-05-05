//! websocket_service.rs
//!
//! There are several places where Alpaca documents the websocket API:
//! 1. https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/realtime/
//! 2. https://alpaca.markets/deprecated/docs/api-documentation/api-v2/streaming/
//! 3. https://alpaca.markets/docs/api-references/trading-api/streaming/
//! 4. https://alpaca.markets/docs/api-references/market-data-api/crypto-pricing-data/realtime/
//!


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
use crate::common::common_structs::{WsListenMessage, WsListenMessageData, MinuteBar, WsAuthenticate};
use crate::models::{AlpWsTrade};
use crate::trader::TraderMsg;
use crate::settings::{STOCK_LIST, STOCK_LIST_COUNT};

#[derive(PartialEq)]
pub enum AlpacaStream{
    TextData,
    BinaryUpdates,
}

fn stock_list_to_uppercase(lower_stock:[&'static str; STOCK_LIST_COUNT])-> Vec<String>{
    lower_stock.map(|x| x.to_uppercase() ).to_vec()
}

pub struct Ws;

impl Ws {
    pub fn run(tx_db: Sender<DbMsg>, tx_trader: Sender<TraderMsg>, stream_type:&AlpacaStream) {
        tracing::debug!("[run]");
        crate::websocket_service::Ws::ws_connect(tx_db, tx_trader, stream_type);
    }



    fn ws_connect(tx_db: Sender<DbMsg>, _tx_trader: Sender<TraderMsg>, stream_type:&AlpacaStream) {

        let ws_url = match stream_type{
            AlpacaStream::TextData => std::env::var("ALPACA_WS_URL_TEXT").expect("ALPACA_WS_URL_TEXT not found"),
            AlpacaStream::BinaryUpdates => std::env::var("ALPACA_WS_URL_BIN").expect("ALPACA_WS_URL_BIN not found"),
        };

        // TODO: handle unwraps
        let url = url::Url::parse(&ws_url).unwrap();
        let request = (&url).into_client_request().unwrap();

        // https://github.com/snapview/tungstenite-rs/blob/master/examples/client.rs
        // tracing::debug!("url: {:?}", &url);
        // tracing::debug!("request: {:?}", &request);

        // commence websocket connection
        match tungstenite::connect(request) {

            Err(e) => tracing::debug!("websocket connect error: {:?}", e),

            Ok((mut ws, _response)) => {

                tracing::debug!("[ws_connect] successful websocket connection; response: {:?}", _response);

                // todo: check if websocket connected; it won't if there's one already connected elsewhere; Alpaca sends an error
                let auth_json = generate_ws_authentication_message();

                // leak keys to docker log with this...
                // tracing::debug!("[ws_connect] sending auth json: {}", &auth_json);

                // send authentication message
                ws.write_message(Message::Text(auth_json)).unwrap();

                loop {
                    // tracing::debug!("[ws_connect] reading websocket...");

                    // non-async version of tungstenite
                    if let Ok(msg) = ws.read_message() {

                        tracing::debug!("[ws_connect] reading something valid from websocket...");

                        match msg {

                            Message::Ping(t) => tracing::debug!("[ws_connect] got ping: {:?}", &t),

                            Message::Binary(t) => {
                                tracing::debug!("[ws_connect][binary] got binary from websocket: {:?}", &t);
                                match serde_json::from_slice::<Value>(&t) {
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
                                            if let Ok(streams) =  serde_json::from_value::<Vec<String>>(json["data"]["streams"].clone()){
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
                            },

                            Message::Text(t_msg) => {
                                tracing::debug!("[ws_connect] read text from websocket: {}",&t_msg);
                                let json_vec: Vec<Value> = serde_json::from_str(&t_msg).unwrap();
                                for json in json_vec {
                                    // https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/realtime/
                                    // [{"T":"success","msg":"connected"}]
                                    // https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/realtime/#minute-bar-schema
                                    // Parse "T" field
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

                                                // Step 1, get successfully connected
                                                if let Some(_msg) = &json["msg"].as_str() {
                                                    tracing::debug!("[ws_connect][text][success] msg: {:?}",&json["msg"].as_str().unwrap());
                                                    match json["msg"].as_str() {
                                                        Some(msg) => {
                                                            // Step 2, get authenticated
                                                            if msg == "authenticated" {
                                                                // subscribe to stock feeds
                                                                // {"action":"subscribe","trades":["AAPL"],"quotes":["AMD","CLDR"],"bars":["AAPL","VOO"]}
                                                                // https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/realtime/#subscribe


                                                                let json = json!({
                                                                    "action": "subscribe",
                                                                    "trades":  stock_list_to_uppercase(STOCK_LIST),
                                                                    // "quotes": STOCK_LIST_CAPS,
                                                                    "bars": stock_list_to_uppercase(STOCK_LIST),
                                                                    // "bars": STOCK_LIST_CAPS,
                                                                });
                                                                tracing::debug!("[ws_connect] sending subscription request...\n{}", &json);
                                                                let result = ws.write_message(Message::Text(json.to_string()));
                                                                tracing::debug!("[ws_connect] subscription request sent: {:?}", &result);

                                                            }
                                                        }
                                                        None => {
                                                            tracing::debug!("[ws_connect][text] success but message was not 'authenticated'");
                                                        }
                                                    }
                                                }
                                            }
                                            "subscription" => {
                                                // subscription confirmation
                                                // [{"T":"subscription","trades":["AAPL"],"quotes":["AMD","CLDR"],"bars":["IBM","AAPL","VOO"]}]
                                                tracing::debug!("[ws_connect][text] subscription confirmation: {:?}",&json);

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
                                                        tracing::debug!("[ws_connect][text] trade parsing failed: {:?}", &e);
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

                                                        tracing::debug!("[ws_connect][text] minute_bar successfully parsed (TODO: send to database): {:?}", &minute_bar);

                                                        let _ = tx_db.send(DbMsg::MinuteBar(minute_bar.to_owned()));
                                                    },
                                                    Err(e) => {
                                                        tracing::debug!("[ws_connect][text] minute_bar parse unsuccessful: {:?}", &e);
                                                    }
                                                }

                                            },
                                            "q" | "d" | "s" => {
                                                // quote, daily, status
                                                tracing::debug!("[ws_connect][text] trade, quote, daily, status: {:?}",&json);
                                            }
                                            _ => {
                                                // tracing::debug!("[ws_connect][ws other] {:?}", &t_msg);
                                            }
                                        }
                                    }
                                }
                            }
                            //
                            // // parse incoming text to json
                            // // let json_val:serde_json::Value = serde_json::from_str(&t).unwrap();
                            // if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&t_msg) {
                            // 	// let json_val = json_val_result.unwrap();
                            //
                            // 	let ws_type = json_val["stream"].as_str();
                            //
                            // 	// tracing::debug!("[run] websocket okay, json_val[stream]: {:?}", ws_type);
                            //
                            // 	match ws_type {
                            // 		Some("T.AAPL") | Some("Q.AAPL") | Some("T.TSLA") | Some("Q.TSLA") |
                            // 		Some("AM.AAPL") | Some("AM.TSLA") | Some("T.PLUG") | Some("Q.PLUG") |
                            // 		Some("AM.PLUG") | Some("T.AAL") | Some("Q.AAL") | Some("AM.AAL") |
                            // 		Some("T.NIO") | Some("Q.NIO") | Some("AM.NIO") | Some("T.BAC") |
                            // 		Some("Q.BAC") | Some("AM.BAC") => {
                            //
                            // 			// tracing::debug!("[run] json_val: {:?}", serde_json::to_string_pretty(&json_val).unwrap());
                            //
                            // 			let json_val = (&json_val)["data"].clone();
                            //
                            // 			match json_val["ev"].as_str() {
                            // 				Some("T") => {
                            //
                            // 					// TODO: remove expect/unwrap
                            // 					// let data:Option<AlpWsTrade> = serde_json::from_value(json_val).expect("[AlpWsTrade] json conversion didn't work");
                            //
                            // 					// let data_result:Result<Option<AlpWsTrade>,serde_json::Error> = serde_json::from_value::<Option<AlpWsTrade>>(json_val); //.expect("[AlpWsTrade] json conversion didn't work");
                            // 					// if let Ok(data) = data_result {
                            // 					if let Ok(data) = serde_json::from_value::<Option<AlpWsTrade>>(json_val) {
                            // 						if let Some(trade) = data {
                            // 							tracing::debug!("[AlpWsTrade] trade: {:?}", &trade);
                            //
                            // 							let _ = tx_trader.send(TraderMsg::TradeWs(trade.clone()));
                            //
                            // 							let _ = tx_db.send(DbMsg::WsTrade(trade.to_owned()));
                            //
                            //
                            // 						} else {
                            // 							tracing::debug!("[AlpWsTrade] trade parsing failed");
                            // 						}
                            // 					}
                            // 				},
                            // 				Some("Q") => {
                            //
                            // 					// let data:Option<AlpWsQuote> = serde_json::from_value(json_val).expect("[AlpWsQuote] json conversion didn't work");
                            // 					if let Ok(data) = serde_json::from_value::<Option<AlpWsQuote>>(json_val) {
                            // 						if let Some(quote) = data {
                            // 							tracing::debug!("[AlpWsTrade] trade: {:?}", &quote);
                            //
                            //
                            //
                            // 							let _ = tx_trader.send(TraderMsg::QuoteWs((&quote).clone()));
                            //
                            // 							let _ = tx_db.send(DbMsg::WsQuote(quote.to_owned()));
                            //
                            // 						} else {
                            // 							tracing::debug!("[AlpWsQuote] trade parsing failed");
                            // 						}
                            // 					}
                            // 				},
                            // 				_ => {}
                            // 			}
                            // 		},
                            // 		Some("listening") => {
                            // 			tracing::debug!("[run] {}", serde_json::to_string_pretty(&json_val).unwrap());
                            // 		},
                            // 		Some("authorization") => {
                            //
                            // 			// use as_str() to remove the quotation marks
                            // 			let json_val = (&json_val)["data"].clone();
                            //
                            // 			// TODO: remove expect()
                            // 			//let data:Option<AlpActionAuthData> = serde_json::from_value(json_val).expect("[authorization] json conversion didn't work"); // unwrap_or(None);
                            //
                            // 			if let Ok(data) = serde_json::from_value::<Option<AlpActionAuthData>>(json_val) {
                            //
                            // 				// tracing::debug!("[run] {:?}", serde_json::to_string_pretty(&data).unwrap());
                            //
                            // 				// if is authorized
                            // 				if let Some(auth) = data {
                            //
                            // 					tracing::debug!("authentication status returned via ws: {:?}", &auth.status);
                            //
                            // 					if auth.status == "authorized" {
                            //
                            // 						// I mean, why do I really care? this is purely informational
                            // 						tracing::debug!("[run] status: authorized");
                            //
                            // 						// Send subscribe "action"
                            // 						let json = Ws::gen_listen_json(vec![
                            // 							"T.AAPL".to_owned(),
                            // 							"Q.AAPL".to_owned(),
                            // 							"T.TSLA".to_owned(),
                            // 							"Q.TSLA".to_owned(),
                            // 							"AM.AAPL".to_owned(),
                            // 							"AM.TSLA".to_owned(),
                            // 							"T.PLUG".to_owned(),
                            // 							"Q.PLUG".to_owned(),
                            // 							"AM.PLUG".to_owned(),
                            // 							"T.AAL".to_owned(),
                            // 							"Q.AAL".to_owned(),
                            // 							"AM.AAL".to_owned(),
                            // 							"T.NIO".to_owned(),
                            // 							"Q.NIO".to_owned(),
                            // 							"AM.NIO".to_owned(),
                            // 							"T.BAC".to_owned(),
                            // 							"Q.BAC".to_owned(),
                            // 							"AM.BAC".to_owned(),
                            //
                            // 						]);
                            //
                            // 						// TODO: tokio::spawn?
                            // 						// let _ = (&mut ws).send(Message::Text(json)).await;
                            // 						let _ = (&mut ws).write_message(Message::Text(json));
                            // 					}
                            // 				}
                            // 			}
                            // 		},
                            // 		_ => {
                            // 			tracing::debug!("[run] Unknown websocket msg: {:?}", json_val);
                            // 		}
                            // 	}
                            // };
                            // }
                            _ => {
                                tracing::debug!("[run] websocket isn't okay, got unrecognizable data: {:?}", &msg);
                            }
                        }
                    } // end double Result on websocket read
                }
            }
        };
    }









}
//
// fn process_message(json_vec:Vec<Value>, tx_db:&Sender<DbMsg>, ws: &mut WebSocket<MaybeTlsStream<TcpStream>>){
//     // alpaca sends every message in an array
//     for json in json_vec {
//
//         // https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/realtime/
//         // [{"T":"success","msg":"connected"}]
//
//         // https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/realtime/#minute-bar-schema
//         // Parse "T" field
//         if let Some(alpaca_msg_type) = json["T"].as_str() {
//             match alpaca_msg_type {
//                 "error" => {
//                     if let Some(_msg) = &json["msg"].as_str() {
//                         tracing::debug!("[ws_connect][ws error] msg: {}({})",&json["msg"].as_str().unwrap(),&json["code"].as_u64().unwrap());
//                     }
//                 }
//                 "success" => {
//
//                     // Step 1, get successfully connected
//
//                     if let Some(_msg) = &json["msg"].as_str() {
//
//                         tracing::debug!("[ws_connect][ws success] msg: {:?}",&json["msg"].as_str().unwrap());
//
//                         match json["msg"].as_str() {
//
//                             Some(msg) => {
//
//                                 // Step 2, get authenticated
//
//                                 if msg == "authenticated" {
//
//                                     // subscribe to stock feeds
//                                     // {"action":"subscribe","trades":["AAPL"],"quotes":["AMD","CLDR"],"bars":["AAPL","VOO"]}
//                                     // https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/realtime/#subscribe
//
//                                     // let json = json!({
//                                     //     "action": "subscribe",
//                                     //     "trades":  STOCK_LIST_CAPS,
//                                     //     // "quotes": stocks,
//                                     //     "bars": STOCK_LIST_CAPS,
//                                     // });
//                                     // let _ = ws.write_message(Message::Text(json.to_string()));
//
//                                     // send a trade_updates subscribe request per:
//                                     // https://alpaca.markets/docs/api-references/trading-api/streaming/
//                                     // should be: {"action":"listen","data":{"streams":["trade_updates"]}}
//                                     // doesn't work: {"action":"listen","data":{"streams":["trade_updates"]}}
//                                     let listen_msg = generate_ws_listen_message(vec!("trade_updates".to_string()));
//                                     tracing::debug!("outgoing listen msg: {}", &listen_msg);
//                                     let _ = ws.write_message(Message::Text(listen_msg));
//
//                                 }
//                             }
//                             None => {
//                                 tracing::debug!("[ws_connect] success but message was not 'authenticated'");
//                             }
//                         }
//                     }
//                 }
//                 "subscription" => {
//                     // subscription confirmation
//                     // [{"T":"subscription","trades":["AAPL"],"quotes":["AMD","CLDR"],"bars":["IBM","AAPL","VOO"]}]
//                     tracing::debug!("subscription confirmation: {:?}",&json);
//
//                     // subscription confirmed; get the latest; change the state machine to accepting updates
//                     // (though not really necessary, can take them if they come)
//                 }
//                 "t" => {
//                     // trade
//                     match serde_json::from_value::<AlpWsTrade>(json) {
//                         Ok(trade) => {
//                             tracing::debug!("[AlpWsTrade] trade: {:?}",&trade);
//                             let _ = tx_db.send(DbMsg::WsTrade(trade.to_owned()));
//
//                         },
//                         Err(e) => {
//                             tracing::debug!("[AlpWsTrade] trade parsing failed: {:?}", &e);
//                         }
//                     }
//                 },
//                 "b" => {
//                     // minute bar schema
//                     // https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/realtime/#minute-bar-schema
//                     /*
//
//                             {
//                               "T": "b",
//                               "S": "SPY",
//                               "o": 388.985,
//                               "h": 389.13,
//                               "l": 388.975,
//                               "c": 389.12,
//                               "v": 49378,
//                               "t": "2021-02-22T19:15:00Z"
//                             }
//
//
//                     */
//                     tracing::debug!("minute bar: {:?}",&json);
//
//                     let minute_bar_result = serde_json::from_value::<MinuteBar>(json);
//
//                     match minute_bar_result {
//                         Ok(minute_bar) => {
//
//                             tracing::debug!("minute_bar successfully parsed (TODO: send to database): {:?}", &minute_bar);
//
//                             let _ = tx_db.send(DbMsg::MinuteBar(minute_bar.to_owned()));
//                         },
//                         Err(e) => {
//                             tracing::debug!("minute_bar parse unsuccessful: {:?}", &e);
//                         }
//                     }
//
//                 },
//                 "q" | "d" | "s" => {
//                     // quote, daily, status
//                     tracing::debug!("trade, quote, daily, status: {:?}",&json);
//                 }
//                 _ => {
//                     // tracing::debug!("[ws_connect][ws other] {:?}", &t_msg);
//                 }
//             }
//         }
//     }
// }


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
fn generate_ws_authentication_message() -> String {
    // {"action": "authenticate","data": {"key_id": "???", "secret_key": "???"}}
    let api_key = std::env::var("ALPACA_API_ID").expect("ALPACA_API_ID");
    let api_secret = std::env::var("ALPACA_API_SECRET").expect("ALPACA_API_SECRET");

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
