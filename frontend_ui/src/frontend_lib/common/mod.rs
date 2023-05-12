//! frontend_lib/common/mod.rs
//!
//!
//!
use chrono::NaiveTime;
use once_cell::sync::Lazy;


pub mod common_structs;
pub mod http;
pub mod trade_struct;

// 2023-04-21T19:46:38.674409Z DEBUG frontend_ui::websocket_service: [ws_connect] read text from websocket: [{"T":"error","code":405,"msg":"symbol limit exceeded"}]

// "celz", "gctk","pearq", "amv",

// TODO: not totally sure if Alpaca API requires all caps
// pub static STOCK_LIST_CAPS:[&'static str; 6] = ["AAL", "AAPL", "BAC", "NIO", "PLUG", "TSLA"];
// pub static STOCK_LIST_CAPS:[&'static str; 22] = STOCK_LIST; // ["AAL", "AAPL", "BAC", "NIO", "PLUG", "TSLA", "SONO", "TGT", "COST", "F", "DIS", "NFLX"];

// https://alpaca.markets/learn/investing-basics/what-is-extended-hours-trading/

// pub static MARKET_EARLY_OPEN_TIME:Lazy<NaiveTime> = Lazy::new(||{ NaiveTime::from_hms_opt(0, 0, 0).unwrap() }); // 4am Eastern
pub static MARKET_OPEN:Lazy<NaiveTime> = Lazy::new(||{ NaiveTime::from_hms_opt(9, 30, 0).unwrap() }); // 4am Eastern
// pub static MARKET_EARLY_OPEN_TIME:Lazy<Date> = Lazy::new(||{ Date::with_hms(9, 30, 0).unwrap() }); // 4am Eastern



// pub static MARKET_EARLY_CLOSE_TIME:Option<NaiveTime> = NaiveTime::from_hms_opt(9, 30, 0);
// pub static MARKET_NORMAL_OPEN_TIME:Option<NaiveTime> = NaiveTime::from_hms_opt(9, 30, 0);
// pub static MARKET_NORMAL_CLOSE_TIME:Option<NaiveTime> = NaiveTime::from_hms_opt(16, 0, 0);
// pub static MARKET_LATE_OPEN_TIME:Option<NaiveTime> = NaiveTime::from_hms_opt(16, 0, 0);

// pub static MARKET_LATE_CLOSE_TIME:Lazy<Date> = Lazy::new(||{ Date::with_hms(16, 0, 0).unwrap() }); // 8pm
pub static MARKET_CLOSE:Lazy<NaiveTime> = Lazy::new(||{ NaiveTime::from_hms_opt(16, 0, 0).unwrap() }); // 8pm