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
    pub(crate) symbol: String,
    #[serde(rename = "o")]
    pub(crate) price_open: BigDecimal,
    #[serde(rename = "h")]
    pub(crate) price_high: BigDecimal,
    #[serde(rename = "l")]
    pub(crate) price_low: BigDecimal,
    #[serde(rename = "c")]
    pub(crate) price_close: BigDecimal,
    #[serde(rename = "v")]
    pub(crate) volume: usize,
    #[serde(rename = "t")]
    pub(crate) dtg: DateTime<Utc>,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct WsAuthenticate {
    pub(crate) action: String,
    pub(crate) key: String,
    pub(crate) secret: String,
}

// { "action": "listen", "data": { "streams": ["T.TSLA", "Q.TSLA", "Q.AAPL", "T.AAPL"]}}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WsListenMessage {
    pub(crate) action: String,
    pub(crate) data: WsListenMessageData,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WsListenMessageData {
    pub(crate) streams: Vec<String>,
}