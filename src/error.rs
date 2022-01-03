use actix_web::{
    HttpRequest, 
    HttpResponse, 
    ResponseError, 
    error::{Error as ActixError, JsonPayloadError},
};
use serde::Serialize;
use sqlx::Error as SqlxError;
use serde_json::json;

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
            Error::NotFound(e) => HttpResponse::NotFound().json(e),
            Error::UnprocessableEntity(e) => HttpResponse::UnprocessableEntity().json(e),
            Error::InternalServerError(e) => HttpResponse::InternalServerError().json(e),
        }
    }
}

impl From<SqlxError> for Error {
    fn from(error: SqlxError) -> Self {
        match &error {
            SqlxError::RowNotFound => Error::NotFound(
                    JsonError { code: 404, status: "NotFound".to_string(), msg: error.to_string()} 
                ),
            SqlxError::ColumnNotFound(s) =>Error::NotFound(
                    JsonError { code: 404, status: "NotFound".to_string(), msg: s.to_string()} 
                ),
            SqlxError::Database(e) => Error::UnprocessableEntity(
                    JsonError { code: 422, status: "UnprocessableEntity".to_string(), msg: e.to_string()} 
                ),
            _ => Error::InternalServerError(
                    JsonError { code: 500, status: "InternalServerError".to_string(), msg: "".to_string()} 
                ),
        }
    }
}

impl Error {
    pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> ActixError {
        let detail = err.to_string();
        let resp = match &err {
            JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(json!( 
                    JsonError { code: 415, status: "UnsupportedMediaType".to_string(), msg: detail }
                )),
            JsonPayloadError::Deserialize(e) if e.is_data() => HttpResponse::UnprocessableEntity().json(json!( 
                    JsonError { code: 422, status: "UnprocessableEntity".to_string(), msg: e.to_string() }
                )),
            _ => HttpResponse::BadRequest().json(json!( 
                    JsonError { code: 400, status: "BadRequest".to_string(), msg: detail } 
                )),
        };
        actix_web::error::InternalError::from_response(err, resp).into()
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
