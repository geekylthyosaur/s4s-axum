use hyper::{Method, Request, Body, header::{CONTENT_TYPE, AUTHORIZATION}};
use serde_json::Value;

use super::TestResult;

#[derive(Default)]
pub struct TestRequest {
    uri: String,
    method: Method,
    json: Option<Value>,
    token: Option<String>,
}

impl TestRequest {
    pub fn build(self) -> TestResult<Request<Body>> {
        let req = Request::builder()
            .method(self.method)
            .uri(self.uri);

        let req = if let Some(token) = self.token {
            req.header(AUTHORIZATION, token)
        } else { req };

        let req = if let Some(json) = self.json {
            req
                .header(CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&json)?))
        } else {
            req.body(Body::empty())
        }?;

        Ok(req)
    }

    pub fn get(uri: impl Into<String>) -> Self {
        Self { uri: uri.into(), method: Method::GET, ..Self::default() }
    }

    pub fn post(uri: impl Into<String>) -> Self {
        Self { uri: uri.into(), method: Method::POST, ..Self::default() }
    }

    pub fn put(uri: impl Into<String>) -> Self {
        Self { uri: uri.into(), method: Method::PUT, ..Self::default() }
    }

    pub fn delete(uri: impl Into<String>) -> Self {
        Self { uri: uri.into(), method: Method::DELETE, ..Self::default() }
    }

    pub fn with_json(self, json: impl Into<Value>) -> Self {
        Self { json: Some(json.into()), ..self }
    }

    pub fn with_auth(self, token: impl Into<String>) -> Self {
        Self { token: Some(token.into()), ..self }
    }
}
