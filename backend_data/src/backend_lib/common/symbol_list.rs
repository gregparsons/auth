//! symbol_list.rs
//!
//! get the active symbols from the database


use sqlx::PgPool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct QrySymbol{
    symbol:String,
}

/// get a vec of stock symbols
///
pub async fn get_symbols(pool:&PgPool)-> Result<Vec<String>,sqlx::Error>{

    let result:Result<Vec<QrySymbol>,sqlx::Error> = sqlx::query_as!(
        QrySymbol,
        r#"select symbol as "symbol!" from t_symbol where active=true"#)
        .fetch_all(pool)
        .await;

    match result {
        Ok(symbol_list) => {
            tracing::debug!("[get_symbols] symbol_list: {:?}", &symbol_list);
            let s = symbol_list.iter().map(|x|{ x.symbol.clone()}).collect();
            Ok(s)
        },
        Err(e)=>{
            tracing::debug!("[get_symbols] error: {:?}", &e);
            Err(e)
        }
    }
}
