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

pub static USERS_ME_JSON_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    let schema = json!({
        "type": "object",
        "properties": {
            "username": { "type": "string" },
            "first_name": { "type": ["string", "null"] },
            "last_name": { "type": ["string", "null"] },
            "email": { "type": "string" },
            "age": { "type": ["number", "null"]},
            "about": { "type": ["string", "null"] },
            "verified": { "type": "boolean" },
            "created_at": { "type": "string" },
            "updated_at": { "type": "string" },
        },
        "required": ["username", "first_name", "last_name", "email", "age", "about", "verified", "created_at", "updated_at"]
    });

    JSONSchema::options().compile(&schema).unwrap()
});

pub static USERS_GET_ALL_JSON_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    let schema = json!({
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "username": { "type": "string" },
                "first_name": { "type": ["string", "null"] },
                "last_name": { "type": ["string", "null"] },
                "email": { "type": "string" },
                "age": { "type": ["number", "null"]},
                "about": { "type": ["string", "null"] },
                "verified": { "type": "boolean" },
                "created_at": { "type": "string" },
                "updated_at": { "type": "string" },
            },
        },
        "required": ["username", "first_name", "last_name", "email", "age", "about", "verified", "created_at", "updated_at"]
    });

    JSONSchema::options().compile(&schema).unwrap()
});

pub static USERS_GET_BY_USERNAME_JSON_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    let schema = json!({
        "type": "object",
        "properties": {
            "username": { "type": "string" },
            "first_name": { "type": ["string", "null"] },
            "last_name": { "type": ["string", "null"] },
            "email": { "type": "string" },
            "age": { "type": ["number", "null"]},
            "about": { "type": ["string", "null"] },
            "verified": { "type": "boolean" },
            "created_at": { "type": "string" },
            "updated_at": { "type": "string" },
        },
        "required": ["username", "first_name", "last_name", "email", "age", "about", "verified", "created_at", "updated_at"]
    });

    JSONSchema::options().compile(&schema).unwrap()
});

impl TestApp {
    pub fn access_token_json_schema() -> &'static JSONSchema {
        &*ACCESS_TOKEN_JSON_SCHEMA
    }

    pub fn users_me_json_schema() -> &'static JSONSchema {
        &*USERS_ME_JSON_SCHEMA
    }

    pub fn users_get_all_json_schema() -> &'static JSONSchema {
        &*USERS_GET_ALL_JSON_SCHEMA
    }

    pub fn users_get_by_username_json_schema() -> &'static JSONSchema {
        &*USERS_GET_BY_USERNAME_JSON_SCHEMA
    }
}
