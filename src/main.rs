use blog::startup::run;
use blog::telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let subscriber = telemetry::get_subscriber("blog".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    let listener = std::net::TcpListener::bind("127.0.0.1:8000").expect("Failed to bind a port");
    run(listener)?.await
}

/*
    1. TODO: single out database configuration
    2. TODO: database -> docker
    5. TODO: single out validation
    6. TODO: randomize tests data
*/
