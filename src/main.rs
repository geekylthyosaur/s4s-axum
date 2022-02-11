use blog::startup::run;
use blog::telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let subscriber = telemetry::get_subscriber("blog".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    let listener =
        std::net::TcpListener::bind("127.0.0.1:8000").expect("Failed to bind a port");
    run(listener)?.await
}
