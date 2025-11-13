use newslatter::prelude::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1: {}", configuration.application_port);

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres.");

    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
