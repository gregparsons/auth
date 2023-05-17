//! src/configuration.rs
//!
//! Ref: Zero 2 Prod page 65
//!
//! TODO: move this to a common library

use config::{ConfigError, File, FileFormat};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

/// generate a postgres connection string from configuration.yaml
impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        let conn_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        );
        // tracing::debug!("[connection_string] {:?}", &conn_string);
        conn_string
    }
}

/// load configuration settings from configuration.toml
/// somewhat inspired by zero2prod but its guidance for config is deprecated (1/16/2023)
pub fn get_yaml_configuration() -> Result<Settings, ConfigError> {

    let config_location = std::env::var("CONFIG_LOCATION").unwrap_or_else(|_| "dev".to_string());

    let config_file_path = match config_location.as_ref() {
        "docker" => "configuration",
        "test" => "config/configuration",
        _ => "frontend/config/configuration",
    };

    // let config_file_path = if config_location {
    //     // Dockerfile says to copy to "."
    //     "configuration"
    // } else {
    //
    //     // TODO: this has to be config/configuration for cargo test -p frontend
    //     // Cargo test doesn't maintain the root directory reference like cargo run and build, apparently
    //     // Cargo workspaces are probably more work than they're worth
    //     "frontend/config/configuration"
    // };

    tracing::debug!(
        "[get_yaml_configuration] config_file_path: {}",
        &config_file_path
    );

    // this file path is different in Docker due to the cargo workspace; kind of annoying
    let settings = config::Config::builder()
        .add_source(File::new(config_file_path, FileFormat::Yaml))
        .build()
        .unwrap();

    // print settings (as a HashMap)
    // doesn't work necessarily using a blank key "database"
    // tracing::debug!(
    //     "{:?}",
    //     (settings.clone())
    //         .try_deserialize::<HashMap<String, String>>()
    //         .unwrap()
    // );

    settings.try_deserialize()
}

#[cfg(test)]
mod tests {
    use crate::configuration::{get_yaml_configuration, DatabaseSettings};

    #[test]
    fn connection_string_still_works() {
        let fake_db_settings = DatabaseSettings {
            username: "postgres".to_string(),
            password: "fakepassword".to_string(),
            port: 54320,
            host: "localhost".to_string(),
            database_name: "alpaca".to_string(),
        };
        let fake_db_conn = "postgres://postgres:fakepassword@localhost:54320/alpaca".to_string();
        assert_eq!(fake_db_settings.connection_string(), fake_db_conn);
    }

    #[test]
    fn yaml_configuration_file_exists() {
        let settings = get_yaml_configuration();
        assert_eq!(
            settings.unwrap().database.database_name,
            "alpaca".to_string()
        );
    }
}
