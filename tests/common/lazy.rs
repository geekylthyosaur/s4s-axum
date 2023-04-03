use jsonschema::JSONSchema;
use once_cell::sync::Lazy;
use s4s::telemetry::Telemetry;
use serde_json::json;

use super::TestApp;

pub static TRACING: Lazy<()> = Lazy::new(|| {
    Telemetry::initialize();
});

pub static ACCESS_TOKEN_JSON_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    let schema = json!({
        "type": "object",
        "properties": {
            "access_token": { "type": "string" },
            "token_type": { "type": "string" }
        },
        "required": ["access_token", "token_type"]
    });

    JSONSchema::options().compile(&schema).unwrap()
});

impl TestApp {
    pub fn access_token_json_schema() -> &'static JSONSchema {
        &*ACCESS_TOKEN_JSON_SCHEMA
    }
}
