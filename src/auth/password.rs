pub fn hash_password(pwd: String) -> String {
    pwd
}

pub fn verify_password(pwd: String, hash: String) -> bool {
    pwd == hash
}
