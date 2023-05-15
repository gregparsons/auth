//! account.rs
//!
//! present account data in the frontend, retrieved from the alpaca API


use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use handlebars::Handlebars;
use reqwest::header::HeaderMap;
use serde_json::json;
use sqlx::PgPool;
use crate::common::common_structs::SESSION_USERNAME;
use crate::common::http::redirect_home;

use serde::{Serialize, Deserialize};
use crate::common::settings::Settings;

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

impl Account {
    pub fn blank() -> Account{
        Account{
            cash: "".to_string(),
            position_market_value: "".to_string(),
            equity: "".to_string(),
            last_equity: "".to_string(),
            daytrade_count: 0,
            balance_asof: "".to_string(),
            pattern_day_trader: false,
            id: "".to_string(),
            account_number: "".to_string(),
            status: "".to_string(),
            currency: "".to_string(),
            buying_power: "".to_string(),
            regt_buying_power: "".to_string(),
            daytrading_buying_power: "".to_string(),
            effective_buying_power: "".to_string(),
            non_marginable_buying_power: "".to_string(),
            bod_dtbp: "".to_string(),
            accrued_fees: "".to_string(),
            pending_transfer_in: "".to_string(),
            portfolio_value: "".to_string(),
            trading_blocked: false,
            transfers_blocked: false,
            account_blocked: false,
            created_at: "".to_string(),
            trade_suspended_by_user: false,
            multiplier: "".to_string(),
            shorting_enabled: false,
            long_market_value: "".to_string(),
            short_market_value: "".to_string(),
            initial_margin: "".to_string(),
            maintenance_margin: "".to_string(),
            last_maintenance_margin: "".to_string(),
            sma: "".to_string()
        }
    }
}

/// GET /account
pub async fn get_account(hb: web::Data<Handlebars<'_>>, pool: web::Data<PgPool>, session:Session) -> impl Responder {

    if let Ok(Some(session_username)) = session.get::<String>(SESSION_USERNAME) {
        tracing::debug!("session id: {}", &session_username);
        let mut headers = HeaderMap::new();

        // let api_key_id = std::env::var("ALPACA_API_ID").expect("ALPACA_API_ID environment variable not found");
        // let api_secret = std::env::var("ALPACA_API_SECRET").expect("alpaca_secret environment variable not found");

        match Settings::load(&pool).await {
            Ok(settings) => {
                let api_key = settings.alpaca_paper_id.clone();
                let api_secret = settings.alpaca_paper_secret.clone();
                headers.insert("APCA-API-KEY-ID", api_key.parse().unwrap());
                headers.insert("APCA-API-SECRET-KEY", api_secret.parse().unwrap());
                let url = format!("https://paper-api.alpaca.markets/v2/account");
                tracing::debug!("[load_fill_activities] calling API: {}", &url);
                // get a single order
                let client = reqwest::Client::new();
                let http_result = client.get(url)
                    .headers(headers)
                    .send()
                    .await;
                let account_body: Account = match http_result {
                    Ok(resp) => {
                        let json_text = &resp.text().await.unwrap();
                        tracing::debug!("json: {}", &json_text);
                        match serde_json::from_str::<Account>(&json_text) {
                            Ok(account) => {
                                tracing::debug!("[get_account] account\n: {:?}", &account);
                                // 3. merge remote results to local database
                                account
                            },
                            Err(e) => {
                                tracing::debug!("[get_account] json: {}", &json_text);
                                tracing::debug!("[get_account] json error: {:?}", &e);
                                Account::blank()
                            }
                        }
                    },
                    Err(e) => {
                        tracing::debug!("[get_account] reqwest error: {:?}", &e);
                        format!("reqwest error: {:?}", &e);
                        Account::blank()
                    }
                };

                // HttpResponse::Ok().body(body)

                let data = json!({
                    "title": "Account",
                    "parent": "base0",
                    "is_logged_in": true,
                    "session_username": &session_username,
                    "message": account_body,
                });
                let body = hb.render("account", &data).unwrap();
                HttpResponse::Ok().append_header(("cache-control", "no-store")).body(body)
            },
            Err(e) => {
                tracing::debug!("[get_account] couldn't load settings (to get alpaca id/secret): {:?}", &e);
                redirect_home().await
            }
        }
    } else {
        redirect_home().await
    }
}