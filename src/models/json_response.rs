use serde::Serialize;

#[derive(Serialize)]
pub struct JsonResponse {
    data: Option<serde_json::Value>,
    error: Option<serde_json::Value>,
}

impl JsonResponse {
    pub fn new(data: Option<serde_json::Value>, error: Option<serde_json::Value>) -> Self {
        JsonResponse { data, error }
    }
}
