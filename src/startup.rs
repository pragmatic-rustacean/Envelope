use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::prelude::*;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            // Middlewares are added using the wrap method on App.
            .wrap(Logger::default())
            .route("/health_check", get().to(health_check))
            .route("/subscriptions", post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)

    //  let connection = Arc::new(Mutex::new(connection));
    //   let server = HttpServer::new(move || {
    //       App::new()
    //           .route("/health_check", web::get().to(health_check))
    //           .route("/subscriptions", web::post().to(subscribe))
    //           // Registering the connection as part of the application state
    //           .app_data(connection.clone())
    //   })
    //   .listen(listener)?
    //   .run();
    //   Ok(server)
}
