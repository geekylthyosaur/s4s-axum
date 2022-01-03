use actix_web::{
    HttpRequest, 
    HttpResponse, 
    ResponseError, 
    error::{Error as ActixError, JsonPayloadError},
};
use serde::Serialize;
use sqlx::Error as SqlxError;
use serde_json::json;

use crate::models::json_response::JsonResponse;

#[derive(Debug, Serialize)]
pub struct JsonError {
    code: u16,
    status: String,
    msg: String,
}

#[derive(Debug)]
pub enum Error {
    NotFound(JsonError),
    UnprocessableEntity(JsonError),
    InternalServerError(JsonError),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::NotFound(e) => 
                HttpResponse::NotFound().json(JsonResponse::new(None, Some(e.to_value()))),
            Error::UnprocessableEntity(e) => 
                HttpResponse::UnprocessableEntity().json(JsonResponse::new(None, Some(e.to_value()))),
            Error::InternalServerError(e) => 
                HttpResponse::InternalServerError().json(JsonResponse::new(None, Some(e.to_value()))),
        }
    }
}

impl From<SqlxError> for Error {
    fn from(error: SqlxError) -> Self {
        match &error {
            SqlxError::RowNotFound => Error::NotFound(
                    JsonError::new(404, "NotFound".to_string(), error.to_string())
                ),
            SqlxError::ColumnNotFound(s) =>Error::NotFound(
                    JsonError::new(404, "NotFound".to_string(), s.to_string())
                ),
            SqlxError::Database(e) => Error::UnprocessableEntity(
                    JsonError::new(422, "UnprocessableEntity".to_string(), e.to_string()) 
                ),
            _ => Error::InternalServerError(
                    JsonError::new(500, "InternalServerError".to_string(), "".to_string())
                ),
        }
    }
}

impl Error {
    pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> ActixError {
        let detail = err.to_string();
        let resp = match &err {
            JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(json!( 
                    JsonError::new(415, "UnsupportedMediaType".to_string(), detail)
                )),
            JsonPayloadError::Deserialize(e) if e.is_data() => HttpResponse::UnprocessableEntity().json(json!( 
                    JsonError::new(422, "UnprocessableEntity".to_string(), e.to_string())
                )),
            _ => HttpResponse::BadRequest().json(json!( 
                    JsonError::new(400, "BadRequest".to_string(), detail)
                )),
        };
        actix_web::error::InternalError::from_response(err, resp).into()
    }
}

impl JsonError {
    fn new(code: u16, status: String, msg: String) -> Self {
        JsonError { code, status, msg }
    }

    fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound(e) => write!(f, "{} {} {}", e.code, e.status, e.msg),
            Error::UnprocessableEntity(e) => write!(f, "{} {} {}", e.code, e.status, e.msg),
            Error::InternalServerError(e) => write!(f, "{} {} {}", e.code, e.status, e.msg),
        }
    }
}
