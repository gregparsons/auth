//! models.rs
//!

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// https://alpaca.markets/docs/api-documentation/api-v2/market-data/last-trade/#last-trade-entity

// {"stream":"authorization","data":{"action":"authenticate","status":"authorized"}}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlpActionAuth {
    stream: String,
    data: AlpActionAuthData,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlpActionAuthData {
    action: String,
    pub(crate) status: String,
}



#[derive(Deserialize, Serialize, Debug, Clone)]
struct AlpacaStreamQuote {
    stream: String,
    data: AlpWsQuote,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct AlpacaStreamTrade {
    stream: String,
    data: AlpWsTrade,
}

/*
    {
        "ev": "T",
        "T": "SPY",
        "i": 117537207,
        "x": 2,
        "p": 283.63,
        "s": 2,
        "t": 1587407015152775000,
        "c": [
        14,
        37,
        41
        ],
        "z": 2
    }
*/

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Trade {
    // #[serde(with = "ts_nanoseconds")]
    #[serde(rename = "t")]
    // pub dtg:DateTime<Utc>, 			// "t": "2021-02-06T13:04:56.334320128Z",
    pub dtg: String,

    #[serde(rename = "x")]
    pub exchange: String, //	"x": "C",

    #[serde(rename = "p")]
    pub price: Decimal, // "p": 387.62,

    #[serde(rename = "s")]
    pub size: u64, // "s": 100,
}

///
/// Ok(Object({"symbol": String("BAC"), "trade": Object({"c": Array([String(" ")]), "i": Number(55359749378617), "p": Number(39.57), "s": Number(100), "t": String("2022-04-12T16:03:26.419177394Z"), "x": String("V"), "z": String("A")})}))
///
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlpacaTradeRest {
    // https://docs.rs/chrono/0.4.19/chrono/serde/ts_nanoseconds/index.html
    #[serde(default)]
    pub symbol: String,
    pub trade: Trade,
    // #[serde(with = "ts_nanoseconds")]
    // #[serde(rename = "timestamp")]
    // pub dtg:DateTime<Utc>,
    // pub price:Decimal,
    // pub size:u64,
    // pub exchange:u64,
    // pub cond1:u64,
    // pub cond2:u64,
    // pub cond3:u64,
    // pub cond4:u64,
    #[serde(default = "Utc::now")]
    pub dtg_updated: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlpWsTrade {
    /*
        Attribute 	Type 	Notes
        T 	string 	message type, always “t”
        S 	string 	symbol
        i 	int 	trade ID
        x 	string 	exchange code where the trade occurred
        p 	number 	trade price
        s 	int 	trade size
        t 	string 	RFC-3339 formatted timestamp with nanosecond precision
        c 	array 	trade condition
        z 	string 	tape
    */
    #[serde(rename = "T")]
    pub event: String,

    // symbol
    #[serde(rename = "S")]
    pub symbol: String,

    #[serde(rename = "i")]
    pub id_trade: usize,

    #[serde(rename = "x")]
    pub exchange: String,

    #[serde(rename = "p")]
    pub price: Decimal,

    #[serde(rename = "s")]
    pub size: usize,

    // #[serde(with = "ts_nanoseconds")]
    #[serde(rename = "t")]
    pub dtg: String, // DateTime<Utc>,

    // #[serde(default)]
    // pub c:Vec<usize>,
    #[serde(rename = "z")]
    pub id_tape: String,

    #[serde(default = "Utc::now")]
    pub dtg_updated: DateTime<Utc>,
}

/*
    {
        "ev": "Q",
        "T": "SPY",
        "x": 17,
        "p": 283.35,
        "s": 1,
        "X": 17,
        "P": 283.4,
        "S": 1,
        "c": [1],
        "t": 1587407015152775000
    }

*/

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlpWsQuote {
    /*
        Attribute 	Type 	Notes
        T 	string 	message type, always “q”
        S 	string 	symbol
        ax 	string 	ask exchange code
        ap 	number 	ask price
        as 	int 	ask size
        bx 	string 	bid exchange code
        bp 	number 	bid price
        bs 	int 	bid size
        s 	int 	trade size
        t 	string 	RFC-3339 formatted timestamp with nanosecond precision
        c 	array 	quote condition
        z 	string 	tape
    */
    #[serde(rename = "T")]
    pub event: String,

    #[serde(rename = "S")]
    pub symbol: String,

    #[serde(rename = "s")]
    pub size_trade: usize,

    // exchange code for bid quote
    #[serde(rename = "bx")]
    pub exchange_bid: usize,

    #[serde(rename = "bp")]
    pub price_bid: Decimal,

    #[serde(rename = "bs")]
    pub size_bid: usize,

    // exchange code for ask quote
    #[serde(rename = "ax")]
    pub exchange_ask: usize,

    #[serde(rename = "ap")]
    pub price_ask: Decimal,

    #[serde(rename = "as")]
    pub size_ask: usize,

    // condition flags
    // pub c:Vec<usize>,

    // timestamp nanoseconds
    // #[serde(with = "ts_nanoseconds")]
    #[serde(rename = "t")]
    pub dtg: String, // DateTime<Utc>,

    #[serde(default = "Utc::now")]
    pub dtg_updated: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Quote {
    pub status: Status,
    pub data: Data,
    /*
    {
        "BTC": Object({
                "circulating_supply": Number(18530887),
                "cmc_rank": Number(1),
                "date_added": String("2013-04-28T00:00:00.000Z"),
                "id": Number(1),
                "is_active": Number(1),
                "is_fiat": Number(0),
                "last_updated": String("2020-11-01T02:41:02.000Z"),
                "max_supply": Number(21000000),
                "name": String("Bitcoin"),
                "num_market_pairs": Number(9191),
                "platform": Null,
                "quote": Object({"USD": Object({"last_updated": String("2020-11-01T02:41:02.000Z"), "market_cap": Number(254545818840.16373), "percent_change_1h": Number(0.14435433), "percent_change_24h": Number(1.0432072), "percent_change_7d": Number(4.47102129), "price": Number(13736.299770224909), "volume_24h": Number(30562293700.698463)})}),
                "slug": String("bitcoin"),
                "symbol": String("BTC"),
                "tags": Array([String("mineable"), String("pow"), String("sha-256"), String("store-of-value"), String("state-channels")]),
                "total_supply": Number(18530887)
            }
        )
    }*/
    // timestamps can be in string here because they insert fine to postgres as an rfc string
    // pub id:i64,
    // pub dtg:chrono::DateTime<chrono::FixedOffset>,

    // #[serde(rename = "timestamp")]
    // pub dtg:String,
    // pub symbol:String,
    // pub price:f64,
    // pub qt_mkt_cap:f64,
    // pub qt_vol_24:f64,
    // pub qt_updated:chrono::DateTime<chrono::FixedOffset>,
    // pub last_updated:String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Status {
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Data {
    #[serde(rename = "BTC")]
    pub btc: CoinType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CoinType {
    pub id: usize,
    pub symbol: String,
    pub is_active: usize,
    pub quote: QuoteCurrency,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QuoteCurrency {
    #[serde(rename = "USD")]
    pub usd: CoinToCurrencyTicker,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CoinToCurrencyTicker {
    pub price: rust_decimal::Decimal,
    pub last_updated: DateTime<Utc>,
}
