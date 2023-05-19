//! symbols.rs
//!
//! get, post a list of stock symbols and whether they're actively traded.
//!

use actix_session::Session;
use actix_web::{HttpResponse, web};
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, Utc};
use handlebars::Handlebars;
use serde_json::json;
use sqlx::PgPool;
use crate::common::http::redirect_home;
use serde::{Serialize,Deserialize};
use crate::common::common_structs::SESSION_USERNAME;
use crate::common::trade_struct::TradeSide;

///
/// GET /symbols
///
pub async fn get_activities(pool: web::Data<PgPool>, hb: web::Data<Handlebars<'_>>, session:Session) -> HttpResponse {
    get_activities_with_message(pool, hb, session, "").await
}

async fn get_activities_with_message(pool: web::Data<PgPool>, hb: web::Data<Handlebars<'_>>, session:Session, message:&str)-> HttpResponse {

    // require login
    if let Ok(Some(session_username)) = session.get::<String>(SESSION_USERNAME) {
        let activity_vec_result = get_activities_from_db(&pool).await;
        match activity_vec_result {
            Ok(activity_vec) => {

                tracing::debug!("[get_activities_with_message] activity_vec: {:?}", &activity_vec);

                let data = json!({
                    "title": "Activity",
                    "parent": "base0",
                    "is_logged_in": true,
                    "session_username": &session_username,
                    "data": &activity_vec,
                    "message": message,
                });
                let body = hb.render("activity_table", &data).unwrap();
                HttpResponse::Ok().append_header(("cache-control", "no-store")).body(body)
            },
            Err(e) => {
                // TODO: redirect to error message
                tracing::debug!("[get_symbols] error getting symbols: {:?}", &e);
                redirect_home().await
            }
        }
    } else {
        redirect_home().await
    }

}




#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityQuery{
    // pub id: String,
    // pub activity_type: ActivityType,
    // fill or partial_fill
    // #[serde(rename="type")]
    // pub activity_subtype: ActivitySubtype,
    pub dtg_utc: NaiveDateTime,
    pub dtg_pacific: NaiveDateTime,
    pub symbol: String,
    pub side: TradeSide,
    pub qty: BigDecimal,
    pub price: BigDecimal,
    // pub cum_qty: BigDecimal,
    // pub leaves_qty: BigDecimal,
    // pub order_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ActivityType{
    #[serde(rename="FILL")]
    Fill
}
#[derive(Deserialize, Serialize, Debug)]
pub enum ActivitySubtype{
    #[serde(rename="fill")]
    Fill,
    #[serde(rename="partial_fill")]
    PartialFill
}



/// get a vec of alpaca trading activities from the postgres database (as a reflection of what's been
/// synced from the Alpaca API)
///
/// TODO: probably already have a common function for this
///
async fn get_activities_from_db(pool:&PgPool) -> Result<Vec<ActivityQuery>,sqlx::Error>{

    // https://docs.rs/sqlx/0.4.2/sqlx/macro.query.html#type-overrides-bind-parameters-postgres-only

    sqlx::query_as!(
    ActivityQuery,
        r#"
            select
                transaction_time::timestamp as "dtg_utc!"
                ,timezone('US/Pacific', transaction_time) as "dtg_pacific!"
                ,symbol as "symbol!"
                ,side as "side!:TradeSide"
                ,qty as "qty!"
                ,price as "price!"
            from alpaca_activity
            order by transaction_time desc
        "#
    )
    .fetch_all(pool)
    .await

}

