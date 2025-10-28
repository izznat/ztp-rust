use secrecy::{ExposeSecret, SecretString};

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub application: ApplicationConfiguration,
    pub database: DatabaseConfiguration,
}

#[derive(serde::Deserialize)]
pub struct ApplicationConfiguration {
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseConfiguration {
    pub user: String,
    pub password: SecretString,
    pub host: String,
    pub port: u16,
    pub name: String,
}

impl DatabaseConfiguration {
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

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let current_directory =
        std::env::current_dir().expect("Failed to determine the current directory.");
    let configuration_directory = current_directory.join("configuration");

    let configuration = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.toml"),
        ))
        .add_source(config::Environment::default().separator("__").prefix("APP"))
        .build()?;

    configuration.try_deserialize::<Configuration>()
}
