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
    pub path: String,
}

impl DatabaseConfiguration {
    pub fn get_database_url(&self) -> String {
        format!("sqlite:{}", self.path)
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
