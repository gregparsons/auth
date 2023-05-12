//! web_server.rs

use crate::configuration::{get_yaml_configuration};
use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;
use sqlx::PgPool;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use crate::frontend::signup::{get_signup, post_signup};
use crate::frontend::login::{get_login, get_logout, post_login};
use crate::frontend::metrics::{get_avg, get_chart};
use crate::frontend::account::get_account;
use crate::frontend::profit::get_profit;
use crate::frontend::utils::*;
use crate::settings::STATIC_FILE_DIR;

pub struct WebServer{}
impl WebServer {

    pub async fn run() {
        let settings = get_yaml_configuration().expect("no configuration.yaml");
        let address = format!("{}:{}", settings.database.host, settings.database.port);
        tracing::debug!("[run] address from config: {}", &address);

        let web_port = settings.application_port;
        tracing::info!("[run] web server starting on port: {}", &web_port);
        let _ = WebServer::web_server(web_port).await;

    }

    fn get_secret_key() -> actix_web::cookie::Key {
        actix_web::cookie::Key::generate()
    }

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
        let secret_key = WebServer::get_secret_key();

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
                .route("/ping", web::get().to(get_ping))
                .route("/avg", web::get().to(get_avg))
                .route("/chart", web::get().to(get_chart))
                .route("/profit", web::get().to(get_profit))
                .route("/account", web::get().to(get_account))
                .route("/logout", web::get().to(get_logout))

        }).bind(("0.0.0.0", web_port))?
            .workers(2)
            .run();

        server.await
    }
}
