//! utils.rs

use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use handlebars::Handlebars;
use serde_json::json;
use crate::common::common_structs::SESSION_USERNAME;
use crate::common::http::redirect_home;

/// authorization: not required
pub async fn get_home(hb: web::Data<Handlebars<'_>>, session:Session) -> HttpResponse {
    tracing::debug!("[get_home]");

    let mut is_logged_in=false;
    let mut cookie_user=String::new();

    if let Ok(Some(session_username)) = session.get::<String>(SESSION_USERNAME) {
        is_logged_in = true;
        cookie_user = session_username;
    }

    // pass username if logged in;
    let data = json!({
        "title": "",
        "parent": "base0",
        "is_logged_in": is_logged_in,
        "session_username": cookie_user,
    });
    let body = hb.render("home", &data).unwrap();

    HttpResponse::Ok()
        .append_header(("Cache-Control", "no-store"))
        .body(body)
}

/// authorization: required
pub async fn get_test1(hb: web::Data<Handlebars<'_>>, session: Session) -> HttpResponse {
    tracing::debug!("[get_test1]");


    // the more concise way to write it
    // todo: still need a way to add authentication by default; ie redirect to login if we forget to do all this authentication
    if let Ok(Some(session_username)) = session.get::<String>(SESSION_USERNAME) {
        let is_logged_in = true;
        tracing::debug!("session id: {}", &session_username);
        let title = format!("test1: {}", &session_username);
        let data = json!({
            "title": &title,
            "parent": "base0",
            "is_logged_in": is_logged_in,
            "session_username": session_username,
        });
        let body = hb.render("template_1", &data).unwrap();
        HttpResponse::Ok()
            .append_header(("Cache-Control", "no-store"))
            .body(body)
    }
    else{
        tracing::debug!("no session id, redirecting to /");
        redirect_home().await
    }
}

/// say "pong"
/// authorization: none
pub async fn get_ping() -> impl Responder {
    tracing::debug!("[get_pong]");
    HttpResponse::Ok().body("pong")
}

