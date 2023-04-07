use assert_json_diff::assert_json_include;
use hyper::{body::HttpBody, Response, StatusCode};
use jsonschema::JSONSchema;
use serde_json::Value;

pub struct Assert<T>(pub Response<T>);

impl<T> Assert<T>
where
    T: HttpBody,
    <T as HttpBody>::Error: std::error::Error,
{
    pub fn status(self, status: StatusCode) -> Self {
        debug_assert_eq!(self.0.status(), status, "{}", self.0.status());
        self
    }

    pub async fn json_schema(self, json_schema: &JSONSchema) {
        let body = hyper::body::to_bytes(self.0.into_body()).await.unwrap();
        let json = serde_json::from_slice(&body).unwrap();
        debug_assert!(json_schema.validate(&json).is_ok(), "{}", json);
    }

    pub async fn json_include(self, expected: Value) {
        let body = hyper::body::to_bytes(self.0.into_body()).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_json_include!(actual: json, expected: expected);
    }

    pub async fn empty_body(self) {
        debug_assert!(hyper::body::to_bytes(self.0.into_body())
            .await
            .unwrap()
            .is_empty());
    }
}
