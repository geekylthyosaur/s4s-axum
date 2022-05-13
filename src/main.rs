use actix_web::{
    web, 
    App, 
    HttpServer
};

use config::{
    configure_app, 
    configure_db, 
};

mod user;
mod config;
mod telemetry;
mod health_check;

#[actix_web::main]
async fn actix_run() -> std::io::Result<()> {
    let pool = match configure_db() {
        Ok(p) => web::Data::new(p),
        Err(e) => panic!("{}", e),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .configure(configure_app)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

fn main() {
    dotenv::dotenv().ok();

    let subscriber = telemetry::get_subscriber("blog".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    match actix_run() {
        Err(e) => panic!("{}", e),
        Ok(_) => (),
    };
}

#[cfg(test)]
mod test_utils {
    use once_cell::sync::Lazy;

    use crate::telemetry;

    static TRACING: Lazy<()> = Lazy::new(|| {
        let subscriber = telemetry::get_subscriber("test".into(), "debug".into());
        telemetry::init_subscriber(subscriber);
    });

    pub fn lazy_init_subscriber() {
        Lazy::force(&TRACING);
    }
}

#[cfg(test)]
mod test_config {
    use sqlx::PgPool;

    pub struct TestDatabaseConfig {
        database_url: String,
    }
    
    impl TestDatabaseConfig {
        pub fn new() -> Self {
            Self {
                database_url: format!("postgres://{}:{}@{}:{}/{}",
                   dotenv::var("DB_USER").expect("Missing DB_USER environment variable"),
                   dotenv::var("DB_PASSWORD").expect("Missing DB_PASSWORD environment variable"),
                   dotenv::var("DB_HOST").expect("Missing DB_HOST environment variable"),
                   dotenv::var("DB_PORT").expect("Missing DB_PORT environment variable"),
                   "blog_test_db")
            }
        }
    }
    
    pub async fn configure_db() -> Result<PgPool, sqlx::Error> {
        // using PgPool::connect_lazy() sometimes there is deadlock
        // so use PgPool::connect() instead
        let pool = PgPool::connect(
            TestDatabaseConfig::new().database_url.as_ref()
        ).await?; 
        sqlx::query!(r#"
            DO $$ DECLARE 
                r RECORD; 
            BEGIN 
                FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = current_schema()) LOOP 
                EXECUTE 
                    'TRUNCATE TABLE ' || quote_ident(r.tablename) || ' CASCADE'; 
                END LOOP; 
            END $$;
        "#)
        .execute(&pool)
        .await
        .expect("Failed to truncate all test database tables!");
        Ok(pool)
    }
}
