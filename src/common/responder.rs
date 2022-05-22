use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::error::{ErrorResponse, SamiStatusCode};
use std::fmt::Debug;

pub type SamiWebResponse<T> = Result<T, ErrorResponse>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SamiResponder<T: Clone + Debug> {
    pub data: Option<T>,
    pub error: Option<ErrorResponse>,
    pub success: bool,
    pub code: SamiStatusCode,
}

impl<T: Clone + Debug> std::fmt::Display for SamiResponder<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {:?}", self)
    }
}

impl<T: Clone + Debug + Serialize> ResponseError for SamiResponder<T> {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserData {
    pub uuid: i32,
    pub email: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub role: Role,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Role {
    HeadBoy,
    NoRole,
}
