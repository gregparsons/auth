//! web_server.rs
//!
//! ref: https://github.com/actix/examples/blob/master/run-in-thread/src/main.rs
//! ref: https://github.com/actix/actix-web/blob/master/actix-web/MIGRATION-4.0.md#server-must-be-polled-warning
//!
//! TODO: This is a rudimentary first draft to get it working without blocking Tokio. Supposedly,
//! in Actix V4 servers can be spawned so need to figure that out and just spawn it under Tokio.
//!
//! TODO: get Actix back to using Tokio...assuming it's not in this spawned thread? Probably
//! better to just create a separate microservice and run a separate docker instance. Keep concerns
//! separate. For now though, as a demo of everything I know in Rust, I'll keep this the mother of all
//! projects.
//!
//!

use crate::configuration::{get_yaml_configuration, Settings};
use actix_web::{rt, web, App, HttpServer};
use handlebars::Handlebars;
use sqlx::PgPool;
use std::thread;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use crate::frontend::signup::{get_signup, post_signup};
use crate::frontend::login::{get_login, get_logout, post_login};
use crate::frontend::metrics::{get_avg, get_chart, get_profit, get_account};
use crate::frontend::utils::*;
use crate::settings::STATIC_FILE_DIR;

/// run()
///
/// spawn a copy of the web server in another thread
///
/// API:
/// http://localhost:8080/ping
pub async fn run(settings: &Settings) {
    tracing::debug!("starting frontend_ui actix web server...");
    let web_port = settings.application_port;
    thread::spawn(move || {
        tracing::info!("inside new web server thread");
        let server_future = web_server(web_port);
        rt::System::new().block_on(server_future)
    });
}

/// TODO: changeme
fn get_secret_key()-> actix_web::cookie::Key{
    actix_web::cookie::Key::generate()
}

/// move server settings to config or env that don't require recompile to change
async fn web_server(web_port: u16) -> std::io::Result<()> {

    tracing::info!("starting HTTP server at http://localhost:8080");

    let configuration = get_yaml_configuration().expect("[web_server] no configuration.yaml?");
    let conn_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("[frontend_ui][web server] failed to connect to postgres");
    let db_pool = web::Data::new(conn_pool);

    // refs:
    // https://github.com/actix/examples/blob/master/templating/handlebars/src/main.rs
    // https://github.com/sunng87/handlebars-rust/tree/master/examples
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", STATIC_FILE_DIR).unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    // srv is server controller type, `dev::Server`
    let secret_key = get_secret_key();

    let server = HttpServer::new(move || {
        App::new()
            // https://actix.rs/docs/middleware
            // setting secure = false for local testing; otherwise TLS required
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(false)
                .build()
            )
            .app_data(db_pool.clone())
            .app_data(handlebars_ref.clone())
            .route("/", web::get().to(get_home))
            .route("/signup", web::get().to(get_signup))
            .route("/login", web::get().to(get_login))
            .route("/signup", web::post().to(post_signup))
            .route("/login", web::post().to(post_login))
            // .route("/test1", web::get().to(get_test1))
            .route("/ping", web::get().to(get_ping))
            .route("/avg", web::get().to(get_avg))
            .route("/chart", web::get().to(get_chart))
            .route("/profit", web::get().to(get_profit))
            .route("/account", web::get().to(get_account))
            .route("/logout", web::get().to(get_logout))

    })
    .bind(("0.0.0.0", web_port))?
    .workers(2)
    .run();

    server.await
}


