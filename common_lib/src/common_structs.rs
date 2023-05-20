//! common_structs.rs

use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

pub static SESSION_USER_ID:&str = "session_user_id";
pub static SESSION_USERNAME:&str = "session_username";

#[derive(Serialize, Deserialize, Debug)]
pub struct FormData{
    pub username:String,
    pub password:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryAverage {
    pub dtg: NaiveDateTime,
    pub symbol: String,
    pub price: BigDecimal,
    pub size: BigDecimal,
    pub exchange: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MinuteBar{
    #[serde(rename = "T")]
    msg_type: String,
    #[serde(rename = "S")]
    pub symbol: String,
    #[serde(rename = "o")]
    pub price_open: BigDecimal,
    #[serde(rename = "h")]
    pub price_high: BigDecimal,
    #[serde(rename = "l")]
    pub price_low: BigDecimal,
    #[serde(rename = "c")]
    pub price_close: BigDecimal,
    #[serde(rename = "v")]
    pub volume: usize,
    #[serde(rename = "t")]
    pub dtg: DateTime<Utc>,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct WsAuthenticate {
    pub action: String,
    pub key: String,
    pub secret: String,
}

// { "action": "listen", "data": { "streams": ["T.TSLA", "Q.TSLA", "Q.AAPL", "T.AAPL"]}}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WsListenMessage {
    pub action: String,
    pub data: WsListenMessageData,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WsListenMessageData {
    pub streams: Vec<String>,
}