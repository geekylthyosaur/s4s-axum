use once_cell::sync::Lazy;

pub static JWT_SECRET: Lazy<String> =
    Lazy::new(|| dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set"));
