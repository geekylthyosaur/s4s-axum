use argon2::{
    password_hash::{rand_core::OsRng, Error, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub fn hash(pwd: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(pwd.as_bytes(), &salt)
        .map(|s| s.to_string())
}

pub fn verify(pwd: &str, hash: &str) -> Result<bool, Error> {
    Ok(Argon2::default()
        .verify_password(pwd.as_bytes(), &PasswordHash::try_from(hash)?)
        .is_ok())
}
