use std::net::TcpListener;

use actix_web::{web::Data, App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::config::{configure_app, configure_db};

pub fn run(listener: TcpListener) -> Result<actix_web::dev::Server, std::io::Error> {
    let pool = match configure_db() {
        Ok(p) => Data::new(p),
        Err(e) => panic!("{}", e),
    };

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(pool.clone())
            .configure(configure_app)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
