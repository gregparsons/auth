//! web_server.rs

use crate::configuration::{get_yaml_configuration};
use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;
use sqlx::PgPool;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use crate::frontend::signup::{get_signup, post_signup};
use crate::frontend::login::{get_login, get_logout, post_login};
use crate::frontend::utils::*;

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
        let conn_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("[frontend][web server] failed to connect to postgres");
        let db_pool = web::Data::new(conn_pool);

        // refs:
        // https://github.com/actix/examples/blob/master/templating/handlebars/src/main.rs
        // https://github.com/sunng87/handlebars-rust/tree/master/examples
        let mut handlebars = Handlebars::new();

        let config_location = std::env::var("CONFIG_LOCATION").unwrap_or_else(|_| "not_docker".to_owned());

        // or CARGO_PKG_NAME
        let package_name = env!("CARGO_MANIFEST_DIR");
        let handlebar_static_path = match config_location.as_str() {
            "docker" => {
                "./static/templates".to_string()
            },
            "not_docker" | _ =>{
                // frontend/static/templates
                // /Users/xyz/.../trade/frontend/static/templates
                format!("{}/static/templates", &package_name)
            }
        };

        tracing::debug!("[web_server] registering handlebars static files to: {}", &handlebar_static_path);

        handlebars.register_templates_directory(".html", handlebar_static_path).unwrap();
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
                .route("/logout", web::get().to(get_logout))

        }).bind(("0.0.0.0", web_port))?
        .workers(2)
        .run();

        server.await
    }
}
