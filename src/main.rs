use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::net::TcpListener;
use ztp::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("ztp".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(&address)?;

    let working_directory =
        std::env::current_dir().expect("Failed to get current working directory.");
    let database_path = working_directory.join(&configuration.database.path);

    if !std::fs::exists(&database_path).expect("Failed to check database file existance.") {
        std::fs::File::create(&database_path).expect("Failed to create empty database file.");
    }

    let mut connect_options = SqliteConnectOptions::new().filename(&database_path);
    unsafe {
        connect_options = connect_options.extension(".sqlpkg/nalgeon/uuid/uuid");
    }
    let connection_pool = SqlitePool::connect_lazy_with(connect_options);

    sqlx::migrate!()
        .run(&connection_pool)
        .await
        .expect("Failed to apply database migrations.");

    run(listener, connection_pool)?.await
}
