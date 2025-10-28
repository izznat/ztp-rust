use secrecy::{ExposeSecret, SecretString};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub port: u16,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub user: String,
    pub password: SecretString,
    pub host: String,
    pub port: u16,
    pub name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        ))
    }

    pub fn connection_string_without_db(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}",
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize our configuration reader.
    let settings = config::Config::builder()
        // Add configuration values from a file named `configuration.yaml`.
        .add_source(config::Environment::default().separator("__").prefix("APP"))
        .build()?;

    // Try to convert configuration values it read into
    // our Settings type.
    settings.try_deserialize::<Settings>()
}
