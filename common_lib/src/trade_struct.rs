//! trade_struct.rs
use std::fmt;
use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonTrade{
    pub(crate) symbol:String,
    pub(crate) side:TradeSide,
    pub(crate) time_in_force:TimeInForce,
    pub(crate) qty:usize,
    #[serde(rename = "type")]
    pub(crate) order_type:OrderType,
    pub(crate) limit_price: Option<BigDecimal>,
    pub(crate) extended_hours: Option<bool>,
}

// https://docs.rs/sqlx/0.4.2/sqlx/macro.query.html#type-overrides-bind-parameters-postgres-only
#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TradeSide{
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "sell_short")]
    SellShort,
}


impl fmt::Display for TradeSide {

    /// enable to_string()
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum TimeInForce{
    #[serde(rename = "gtc")]
    Gtc,
    #[serde(rename = "day")]
    Day,
    // Immediate or Cancel
    #[serde(rename = "ioc")]
    Ioc,



}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderType{
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "limit")]
    LIMIT,
}