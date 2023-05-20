//! settings.rs
//!
//! model for settings store in postgres db

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub dtg:DateTime<Utc>,
    pub alpaca_paper_id:String,
    pub alpaca_paper_secret:String,
    pub alpaca_live_id:String,
    pub alpaca_live_secret:String,
    pub trade_size:i32,
    pub trade_enable_buy:bool,
    pub trade_ema_small_size:i32,
    pub trade_ema_large_size:i32,
    pub trade_sell_high_per_cent_multiplier:BigDecimal,
    pub trade_sell_high_upper_limit_cents:BigDecimal
}

impl Settings {

    ///
    /// TODO: encrypt alpaca credentials in database and decrypt here using .env
    ///
    pub async fn load(pool:&PgPool)->Result<Settings, sqlx::Error>{

        let settings_result = sqlx::query_as!(
            Settings,
            r#"
                select
                    dtg as "dtg!"
                    , alpaca_paper_id as "alpaca_paper_id!"
                    , alpaca_paper_secret as "alpaca_paper_secret!"
                    , alpaca_live_id as "alpaca_live_id!"
                    , alpaca_live_secret as "alpaca_live_secret!"
                    , trade_size as "trade_size!"
                    , trade_enable_buy as "trade_enable_buy!"
                    , trade_ema_small_size as "trade_ema_small_size!"
                    , trade_ema_large_size as "trade_ema_large_size!"
                    , trade_sell_high_per_cent_multiplier as "trade_sell_high_per_cent_multiplier!"
                    , trade_sell_high_upper_limit_cents as "trade_sell_high_upper_limit_cents!"
                from v_settings;
            "#
        ).fetch_one(pool).await;

        // don't spill credentials to log
        // tracing::debug!("[settings::load] {:?}", &settings_result);

        settings_result

    }
}