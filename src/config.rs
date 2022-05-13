use actix_web::web;
use sqlx::PgPool;

use crate::health_check::health_check;
use crate::user::signup;

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                web::scope("/health_check")
                    .service(web::resource("").route(web::get().to(health_check))),
            )
            .service(
                web::scope("/users")
                    .service(web::resource("").route(web::post().to(signup::signup))),
            ),
    );
}

pub struct DatabaseConfig {
    database_url: String,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        Self {
            database_url: format!("postgres://{}:{}@{}:{}/{}",
               dotenv::var("DB_USER").expect("Missing DB_USER environment variable"),
               dotenv::var("DB_PASSWORD").expect("Missing DB_PASSWORD environment variable"),
               dotenv::var("DB_HOST").expect("Missing DB_HOST environment variable"),
               dotenv::var("DB_PORT").expect("Missing DB_PORT environment variable"),
               dotenv::var("DB_NAME").expect("Missing DB_NAME environment variable"))
        }
    }
}

pub fn configure_db() -> Result<PgPool, sqlx::Error> {
    PgPool::connect_lazy(
        &DatabaseConfig::new().database_url
    )
}

#[cfg(test)]
pub mod test_config {
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
