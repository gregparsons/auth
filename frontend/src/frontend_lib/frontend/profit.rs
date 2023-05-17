//! profit.rs
//!

use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, Utc};
use handlebars::Handlebars;
use serde_json::json;
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

use crate::common::common_structs::SESSION_USERNAME;
use crate::common::http::redirect_home;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryProfit {
    pub symbol: String,
    pub profit_to_date: BigDecimal,
    pub activity_count: BigDecimal,
    pub count_today: BigDecimal,
    pub count_yesterday: BigDecimal,
    pub price_avg: BigDecimal,
    pub volume: BigDecimal,
    pub profit_vs_activities: BigDecimal,
    pub profit_vs_price: BigDecimal,
    pub profit_vs_volume: BigDecimal,
    pub trade_latest: NaiveDateTime,
    pub activity_latest: DateTime<Utc>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryProfitTotal {
    pub profit: BigDecimal,
}

/// GET /profit
/// print a table of stocks P/L
pub async fn get_profit(hb: web::Data<Handlebars<'_>>, db_pool: web::Data<PgPool>, session:Session) -> impl Responder {

    tracing::debug!("[get_profit]");

    if let Ok(Some(session_username)) = session.get::<String>(SESSION_USERNAME) {
        tracing::debug!("session id: {}", &session_username);

        // exclamation point means we're overriding sqlx requiring Option<> on nullables (assuming we know it'll never be null)
        let profit_vec = match sqlx::query_as!(
            QueryProfit,
                r#"
                    select * from v_stats
                "#,
            ).fetch_all(db_pool.as_ref()).await {

                Ok(vec_of_profit) => vec_of_profit,

                Err(e) => {
                    tracing::debug!("[get_profit] profit report error: {:?}", &e);
                    vec![]
                }
        };

        // let json_string = json!(profit_vec).to_string();
        // tracing::debug!("[get_profit] profit report:\n{:?}", &json_string);

        let prof_ttl:QueryProfitTotal = match sqlx::query_as!(
            QueryProfitTotal,
                r#"
                    select sum("subtotal!") as "profit!" from v_profit
                "#,
            )
            .fetch_one(db_pool.as_ref())
            .await {

                    Ok(one) => one,
                    Err(_e) => {
                        QueryProfitTotal{
                            profit: BigDecimal::from(0),
                        }
                    },
            };


        tracing::debug!("{:?}", &prof_ttl);

        let data = json!({
            "title": "Profit",
            "parent": "base0",
            "is_logged_in": true,
            "session_username": &session_username,
            "data": profit_vec,
            "profit_total": prof_ttl
        });

        let body = hb.render("profit_table", &data).unwrap();
        // tracing::debug!("[get_profit] body: {:?}", &body);
        HttpResponse::Ok().append_header(("cache-control", "no-store")).body(body)

    } else {
        redirect_home().await
    }
}