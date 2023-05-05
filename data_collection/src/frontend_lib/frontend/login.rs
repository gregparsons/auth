//! login.rs

use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use handlebars::Handlebars;
use serde_json::json;
use sqlx::PgPool;
use sqlx::types::Uuid;
use crate::common::common_structs::{FormData, SESSION_USER_ID, SESSION_USERNAME};
use crate::common::http::redirect_home;

/// curl http://localhost:8080/login
/// authorization: TBD
pub async fn get_login(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    tracing::debug!("[get_login]");
    let data = json!({
        "title": "Login",
        "parent": "base0",

    });
    let body = hb.render("login", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[derive(Debug, Clone)]
struct LoginResult{
    user_id:Uuid,
    username:String,
    password:String,
}

pub async fn post_login(form: web::Form<FormData>, hb: web::Data<Handlebars<'_>>, db_pool: web::Data<PgPool>, session: Session) -> HttpResponse {
    tracing::debug!("[post_login] {:?}", &form);

    let login_result:Result<LoginResult, sqlx::Error> = sqlx::query_as!(
        LoginResult,
        r#"
            select
                user_id, username, password
            from users
            where username = $1
        "#, form.username
    ).fetch_one(db_pool.as_ref()).await;
    tracing::debug!("[post_login] login_result: {:?}", &login_result);


    let login_result_unwrapped = &login_result.unwrap();

    let stored_pw_hash = &login_result_unwrapped.password;

    let correct_pw_hash = PasswordHash::new(stored_pw_hash).expect("couldn't convert password hash stored in database to a proper Argon hash");

    let is_verified = Argon2::default().verify_password(
        form.password.as_bytes(),
        &correct_pw_hash
    );

    match is_verified {
        Ok(_verified) => {
            tracing::debug!("[post_login] is_verified: yes!");
            let _result = session.insert(SESSION_USER_ID, &login_result_unwrapped.user_id.to_string());
            let _result = session.insert(SESSION_USERNAME, &login_result_unwrapped.username);
            let message = format!("Welcome, {}, ({})", &login_result_unwrapped.username, &login_result_unwrapped.user_id);

            let data = json!({
                "title": "login",
                "parent": "base0",
                "is_logged_in": true,
                "session_username": &login_result_unwrapped.username,
                "message": &message,
            });
            let body = hb.render("login_result", &data).unwrap();

            HttpResponse::Ok().append_header(("Cache-Control", "no-store")).body(body)


        },
        Err(e)=> {
            tracing::debug!("[post_login] not verified, error: {:?}", &e);
            // let message = "Sorry. Login failed.".to_string();
            redirect_home().await
        }
    }
}

pub async fn get_logout(session:Session) -> impl Responder {
    session.purge();
    redirect_home().await
}