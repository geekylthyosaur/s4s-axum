use hyper::StatusCode;
use jsonschema::JSONSchema;
use serde_json::Value;

pub struct Assert(pub StatusCode, pub Value);

impl Assert {
    pub fn status(self, status: StatusCode) -> Self {
        debug_assert_eq!(self.0, status, "{}", self.1.to_string());
        self
    }

    pub fn json_body_with_schema(self, json_schema: &JSONSchema) -> Self {
        debug_assert!(
            json_schema.validate(&self.1).is_ok(),
            "{}",
            self.1.to_string()
        );
        self
    }
}
