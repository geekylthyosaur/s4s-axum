use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Score {
    pub uuid: Uuid,
    pub score: i32,
}

#[derive(Deserialize)]
pub struct ScoreToCreate {
    pub uuid: Uuid,
}

impl Score {
    pub fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
