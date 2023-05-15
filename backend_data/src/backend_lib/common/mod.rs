//! backend_lib/common/mod.rs

use chrono::NaiveTime;
use once_cell::sync::Lazy;
pub mod common_structs;
pub mod http;
pub mod trade_struct;
pub mod sqlx_pool;
pub mod symbol_list;
pub mod settings;

// 2023-04-21T19:46:38.674409Z DEBUG frontend_ui::websocket_service: [ws_connect] read text from websocket: [{"T":"error","code":405,"msg":"symbol limit exceeded"}]
// https://alpaca.markets/learn/investing-basics/what-is-extended-hours-trading/

pub static MARKET_OPEN_TIME:Lazy<NaiveTime> = Lazy::new(||{ NaiveTime::from_hms_opt(9, 30, 0).unwrap() }); // 4am Eastern
pub static MARKET_CLOSE_TIME:Lazy<NaiveTime> = Lazy::new(||{ NaiveTime::from_hms_opt(16, 0, 0).unwrap() }); // 8pm
