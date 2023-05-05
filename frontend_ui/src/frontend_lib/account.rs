//! account.rs
//!
//! https://alpaca.markets/docs/api-references/trading-api/account/
//!
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub(crate) cash: String,
    pub(crate) position_market_value: String,
    pub(crate) equity: String,
    pub(crate) last_equity: String,
    pub(crate) daytrade_count: usize,
    pub(crate) balance_asof: String,
    pub(crate) pattern_day_trader: bool,
    pub(crate) id: String,
    pub(crate) account_number: String,
    pub(crate) status: String,
    // crypto_status: String,
    pub(crate) currency: String,
    pub(crate) buying_power: String,
    pub(crate) regt_buying_power: String,
    pub(crate) daytrading_buying_power: String,
    pub(crate) effective_buying_power: String,
    pub(crate) non_marginable_buying_power: String,
    pub(crate) bod_dtbp: String,
    pub(crate) accrued_fees:String,
    pub(crate) pending_transfer_in: String,
    pub(crate) portfolio_value: String,
    pub(crate) trading_blocked: bool,
    pub(crate) transfers_blocked: bool,
    pub(crate) account_blocked: bool,
    pub(crate) created_at: String,
    pub(crate) trade_suspended_by_user: bool,
    pub(crate) multiplier: String,
    pub(crate) shorting_enabled: bool,
    pub(crate) long_market_value: String,
    pub(crate) short_market_value: String,
    pub(crate) initial_margin: String,
    pub(crate) maintenance_margin: String,
    pub(crate) last_maintenance_margin: String,
    pub(crate) sma: String,
    // crypto_tier: usize,
}

