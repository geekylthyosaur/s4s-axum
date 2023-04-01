pub fn hash_password(pwd: &str) -> String {
    pwd.to_owned()
}

pub fn verify_password(pwd: &str, hash: &str) -> bool {
    pwd == hash
}
