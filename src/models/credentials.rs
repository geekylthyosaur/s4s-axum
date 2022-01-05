use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub id: i32,
    pub owner_id: i32,
    pub email: String,
    pub is_email_confirmed: bool,
    pub pwd_hash: String,
}

#[derive(Deserialize)]
pub struct CredentialsToUpdate {
    pub email: String,
    pub pwd_hash: String,
}

impl Credentials {
    pub fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
