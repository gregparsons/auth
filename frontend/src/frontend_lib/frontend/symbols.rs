//! symbols.rs
//!
//! get, post a list of stock symbols and whether they're actively traded.
//!



use actix_session::Session;
use actix_web::{HttpResponse, web};
use actix_web::web::Form;
use handlebars::Handlebars;
use serde_json::json;
use sqlx::PgPool;
use crate::common::http::redirect_home;
use serde::{Serialize,Deserialize};
use crate::common::common_structs::SESSION_USERNAME;

///
/// POST /symbols
///
pub async fn post_symbols(form: Form<ActiveSymbol>, pool: web::Data<PgPool>, hb: web::Data<Handlebars<'_>>, session:Session) -> HttpResponse {

    tracing::debug!("[post_symbols]");
    // require login
    if let Ok(Some(_session_username)) = session.get::<String>(SESSION_USERNAME) {
        let ActiveSymbol{ symbol, active }  = form.into_inner();

        // TODO; update t_symbol set active=true where symbol = 'arvl';

        let result_message = match save_symbol_active(&pool, &symbol, active).await {
            None => {
                // no problem return
                "Symbol change saved"
            },
            Some(e)=>{
                tracing::debug!("[post_symbols] error saving symbol/active: {:?}", &e);
                "Error saving symbol"
            }
        };

        get_symbols_with_message(pool, hb, session, format!("{} [{}]", result_message, &symbol).as_str()).await

    } else {
        // not logged in
        redirect_home().await
    }


}

///
/// GET /symbols
///
pub async fn get_symbols(pool: web::Data<PgPool>, hb: web::Data<Handlebars<'_>>, session:Session) -> HttpResponse {

    get_symbols_with_message(pool, hb, session, "").await

}

async fn get_symbols_with_message(pool: web::Data<PgPool>, hb: web::Data<Handlebars<'_>>, session:Session, message:&str)-> HttpResponse {

    // require login
    if let Ok(Some(session_username)) = session.get::<String>(SESSION_USERNAME) {
        let symbol_vec_result = get_symbols_with_active(&pool).await;
        match symbol_vec_result {
            Ok(symbol_vec) => {
                let data = json!({
                "title": "Symbols",
                "parent": "base0",
                "is_logged_in": true,
                "session_username": &session_username,
                "data": &symbol_vec,
                "message": message,
            });
                let body = hb.render("symbol_table", &data).unwrap();
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveSymbol{
    symbol:String,
    active:bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveSymbol2{
    symbol:String,
    active:String,
}

/// get a vec of stock symbols
///
async fn get_symbols_with_active(pool:&PgPool)-> Result<Vec<ActiveSymbol>,sqlx::Error>{

    sqlx::query_as!(
    ActiveSymbol,
        r#"
            select
                symbol as "symbol!"
             , active as "active!"
            from t_symbol
            order by symbol
        "#
    )
    .fetch_all(pool)
    .await

}

/// save a change to the symbol's active status
async fn save_symbol_active(pool: &PgPool, symbol:&str, active:bool) -> Option<sqlx::Error> {
    match sqlx::query!(
        r#"
            update t_symbol set active = $2 where symbol = $1
        "#,
        symbol,
        active
    )
    .execute(pool)
    .await {
        Ok(_) =>{
            None
        },
        Err(e) => {
            tracing::debug!("[save_symbol_active] error saving symbol and active field: {:?}", &e);
            Some(e)
        }
    }

}