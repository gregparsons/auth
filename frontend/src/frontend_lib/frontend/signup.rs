//! signup.rs

use actix_web::{HttpResponse, web};
use handlebars::Handlebars;
use serde_json::json;
use common_lib::common_structs::FormData;
use argon2::password_hash::SaltString; use argon2::{Argon2, PasswordHasher};
use sqlx::PgPool;
use sqlx::types::Uuid;

/// authorization: none
pub async fn get_signup(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    tracing::debug!("[get_signup]");
    let data = json!({
        "title": "Signup",
        "parent": "base0"
    });
    let body = hb.render("signup", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[derive(Debug, Clone)]
struct SignupResult{
    user_id:Uuid,
}

/// ref: p. 360 zero2prod
pub async fn post_signup(form: web::Form<FormData>, hb: web::Data<Handlebars<'_>>, pool: web::Data<PgPool>) -> HttpResponse {

    tracing::debug!("[post_signup] {:?}", &form);

    let salt = SaltString::generate(&mut rand::thread_rng()); // We don't care about the exact Argon2 parameters here // given that it's for testing purposes!
    let password_hash = Argon2::default()
        .hash_password(form.password.as_bytes(), &salt) .unwrap()
        .to_string();

    let signup_result:Result<SignupResult, sqlx::Error> = sqlx::query_as!(
        SignupResult,
        r#"
            with rows as (
                insert into users (
                    user_id, username, password
                )
                values (gen_random_uuid(), $1, $2)
                returning user_id
            ) select user_id from rows
        "#, form.username, password_hash
    ).fetch_one(pool.as_ref())
    .await;

    let message = match signup_result {
        Ok(signup) => {
            tracing::debug!("[post_signup] just signed up user id: {}", &signup.user_id);
            format!("Success. Signed up user id: {}", &signup.user_id)
        },
        Err(e) => {
            // todo: extract Postgres error
            tracing::debug!("[post_signup] signup failed: {:?}", &e);
            format!("Signup failed. Shrug emoji?")
        }
    };



    // todo: result...


    let data = json!({
        "title": "signup result",
        "parent": "base0",
        "message": &message,
    });
    let body = hb.render("message", &data).unwrap();
    HttpResponse::Ok().body(body)
}