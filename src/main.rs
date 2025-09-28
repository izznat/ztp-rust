use std::net::TcpListener;
use ztp::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.port);
    let listener = TcpListener::bind(&address)?;
    run(listener)?.await
}
