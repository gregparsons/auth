//! frontend_lib/common/mod.rs
//!
//!
//!
use chrono::NaiveTime;
use once_cell::sync::Lazy;

// https://alpaca.markets/learn/investing-basics/what-is-extended-hours-trading/
pub static MARKET_OPEN:Lazy<NaiveTime> = Lazy::new(||{ NaiveTime::from_hms_opt(9, 30, 0).unwrap() }); // 4am Eastern
pub static MARKET_CLOSE:Lazy<NaiveTime> = Lazy::new(||{ NaiveTime::from_hms_opt(16, 0, 0).unwrap() }); // 8pm