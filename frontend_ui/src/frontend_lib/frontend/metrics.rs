//! metrics.rs

use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use bigdecimal::BigDecimal;
use handlebars::Handlebars;
use reqwest::header::HeaderMap;
use serde::{Serialize,Deserialize};
use serde_json::json;
use sqlx::PgPool;
use crate::account::Account;
use crate::common::common_structs::{QueryAverage, SESSION_USERNAME};
use crate::common::http::redirect_home;

/// authorization: required
pub async fn get_avg(hb: web::Data<Handlebars<'_>>, db_pool: web::Data<PgPool>, session:Session) -> impl Responder {

    if let Ok(Some(session_username)) = session.get::<String>(SESSION_USERNAME) {
        tracing::debug!("session id: {}", &session_username);
        let json = get_data(&db_pool).await;

        let data = json!({
            "title": "1-minute ticker",
            "parent": "base0",
            "is_logged_in": true,
            "session_username": &session_username,
            "data"  : json
        });
        let body = hb.render("avg", &data).unwrap();
        HttpResponse::Ok()
            .append_header(("cache-control", "no-store"))
            .body(body)
    }else{
        redirect_home().await
    }

}

/// authorization: required
pub async fn get_chart(hb: web::Data<Handlebars<'_>>, db_pool: web::Data<PgPool>, session:Session) -> impl Responder {
    tracing::debug!("[get_chart]");
    if let Ok(Some(session_username)) = session.get::<String>(SESSION_USERNAME) {
        tracing::debug!("session id: {}", &session_username);
        let json = get_data(&db_pool).await;
        let data = json!({
            "title": "1-Minute Chart",
            "parent": "base0",
            "is_logged_in": true,
            "session_username": &session_username,
            "data"  : json
        });
        let body = hb.render("chart", &data).unwrap();
        HttpResponse::Ok().append_header(("cache-control", "no-store")).body(body)
    } else {
        redirect_home().await
    }
}


/// placeholder
///
/// json of Vec<QueryAverage>
async fn get_data(db_pool: &web::Data<PgPool>) -> String{

    // exclamation point means we're overriding sqlx requiring Option<> on nullables (assuming we know it'll never be null)
    let result_vector = sqlx::query_as!(
        QueryAverage,
        r#"
            select
                dtg as "dtg!",
                coalesce(symbol, '') as "symbol!",
                price as "price!",
                size as "size!",
                exchange as "exchange"
            from
                v_trade_minute
            order by dtg desc
            limit 1000
        "#,
    )
        .fetch_all(db_pool.as_ref())
        .await
        .unwrap();

    json!(result_vector).to_string()

}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryProfit {
    pub symbol: String,
    pub qty: BigDecimal,
    pub subtotal: BigDecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryProfitTotal {
    pub profit: BigDecimal,
}

/// GET /profit
/// print a table of stocks P/L
pub async fn get_profit(hb: web::Data<Handlebars<'_>>, db_pool: web::Data<PgPool>, session:Session) -> impl Responder {

    if let Ok(Some(session_username)) = session.get::<String>(SESSION_USERNAME) {
        tracing::debug!("session id: {}", &session_username);


        // exclamation point means we're overriding sqlx requiring Option<> on nullables (assuming we know it'll never be null)
        let profit_vec = match sqlx::query_as!(
            QueryProfit,
                r#"
                    select
                        "symbol!", "qty!", "subtotal!"
                    from v_profit
                "#,
            )
            .fetch_all(db_pool.as_ref())
            .await {

            Ok(vec_of_profit) => vec_of_profit,
            Err(_e) => vec![]
        };


        // let json_string = json!(result_vector).to_string();
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



/// GET /account
pub async fn get_account(hb: web::Data<Handlebars<'_>>, _db_pool: web::Data<PgPool>, session:Session) -> impl Responder {

    if let Ok(Some(session_username)) = session.get::<String>(SESSION_USERNAME) {
        tracing::debug!("session id: {}", &session_username);


        let mut headers = HeaderMap::new();
        let api_key_id = std::env::var("ALPACA_API_ID").expect("ALPACA_API_ID environment variable not found");
        let api_secret = std::env::var("ALPACA_API_SECRET").expect("alpaca_secret environment variable not found");
        headers.insert("APCA-API-KEY-ID", api_key_id.parse().unwrap());
        headers.insert("APCA-API-SECRET-KEY", api_secret.parse().unwrap());

        let url = format!("https://paper-api.alpaca.markets/v2/account");

        tracing::debug!("[load_fill_activities] calling API: {}", &url);

        // get a single order
        let client = reqwest::Client::new();

        let http_result = client.get(url)
            .headers(headers)
            .send()
            .await;

        let account_body:Account = match http_result {

            Ok(resp) => {

                let json_text = &resp.text().await.unwrap();

                tracing::debug!("json: {}", &json_text);

                match serde_json::from_str::<Account>(&json_text) {
                    Ok(account) => {

                        tracing::debug!("[get_account] account\n: {:?}", &account);

                        // 3. merge remote results to local database
                        // let _result = a.save_to_db(&pool).await;

                        // TODO: pass this to handlebars
                        // account
                        // format!("cash: {}\nposition_market_value: {}\nequity: {}\ndaytrade_count: {}",
                        //         account.cash,
                        //         account.position_market_value,
                        //         account.equity,
                        //         account.daytrade_count)
                        account

                    },
                    Err(e) => {
                        tracing::debug!("[get_account] json: {}", &json_text);
                        tracing::debug!("[get_account] json error: {:?}", &e);

                        // format!("{}",&json_text)
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
            },
            Err(e) => {
                tracing::debug!("[get_account] reqwest error: {:?}", &e);
                format!("reqwest error: {:?}",&e);
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

        // tracing::debug!("[get_profit] body: {:?}", &body);

        HttpResponse::Ok().append_header(("cache-control", "no-store")).body(body)

    } else {
        redirect_home().await
    }


}