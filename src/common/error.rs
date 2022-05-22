use super::responder::SamiResponder;
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tokio_postgres::Error as PostgresError;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SamiErrorWithData {
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for SamiErrorWithData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "field: {}, message:{}", self.field, self.message)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Display, PartialEq)]
pub enum SamiError {
    BadRequest(SamiErrorWithData),
    #[display(fmt = "{:?}", field)]
    InternalError {
        field: String,
    },
    SqlError {
        field: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub field: Option<String>,
    pub message: Option<String>,
    pub code: SamiStatusCode,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum SamiStatusCode {
    OK,
    Bad,
    Internal,
    Sql,
    NotFound,
    ExpectationFailed,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {:?}", self)
    }
}

impl Into<ErrorResponse> for SamiError {
    fn into(self) -> ErrorResponse {
        match self {
            SamiError::BadRequest(e) => ErrorResponse {
                field: Some(e.field),
                message: Some(e.message),
                code: SamiStatusCode::Bad,
            },
            SamiError::InternalError { field } => ErrorResponse {
                field: None,
                message: Some(field),
                code: SamiStatusCode::Internal,
            },
            SamiError::SqlError { field } => ErrorResponse {
                field: None,
                message: Some(field),
                code: SamiStatusCode::Sql,
            },
        }
    }
}

impl Into<ErrorResponse> for PostgresError {
    fn into(self) -> ErrorResponse {
        match self.as_db_error() {
            Some(res) => ErrorResponse {
                field: res.column().map(|e| e.to_string()),
                message: res.detail().map(|e| e.to_string()),
                code: SamiStatusCode::Sql,
            },
            None => ErrorResponse {
                field: None,
                message: Some(format!(
                    "DB unable to process the result / not found, {:?}",
                    self.into_source()
                )),
                code: SamiStatusCode::Sql,
            },
        }
    }
}

impl<T: Clone + Debug> Into<SamiResponder<T>> for ErrorResponse {
    fn into(self) -> SamiResponder<T> {
        SamiResponder {
            data: None,
            error: Some(self.clone()),
            success: false,
            code: self.code,
        }
    }
}

impl ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .json(self)
    }
    fn status_code(&self) -> StatusCode {
        match &self.code {
            SamiStatusCode::OK => StatusCode::OK,
            SamiStatusCode::Bad => StatusCode::BAD_REQUEST,
            SamiStatusCode::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            SamiStatusCode::Sql => StatusCode::UNPROCESSABLE_ENTITY,
            SamiStatusCode::NotFound => StatusCode::NOT_FOUND,
            SamiStatusCode::ExpectationFailed => StatusCode::EXPECTATION_FAILED,
        }
    }
}

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "JWKSFetchError")]
    JWKSFetchError,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::JWKSFetchError => {
                HttpResponse::InternalServerError().json("Could not fetch JWKS")
            }
        }
    }
}
