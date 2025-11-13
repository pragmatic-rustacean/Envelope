use newslatter::prelude::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1: {}", &configuration.database.port);

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres.");

    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
