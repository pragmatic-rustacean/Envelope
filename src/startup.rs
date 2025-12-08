use crate::prelude::*;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            // Middlewares are added using the wrap method on App.
            .wrap(TracingLogger::default())
            .route("/health_check", get().to(health_check))
            .route("/subscriptions", post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
