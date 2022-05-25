use super::responder::SamiResponder;
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tokio_postgres::Error as PostgresError;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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
    AuthenticationFailed,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {:?}", self)
    }
}

impl Into<ErrorResponse> for PostgresError {
    fn into(self) -> ErrorResponse {
        ErrorResponse {
            field: None,
            message: Some(self.to_string()),
            code: SamiStatusCode::Sql,
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
            SamiStatusCode::AuthenticationFailed => StatusCode::UNAUTHORIZED,
        }
    }
}