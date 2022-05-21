use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use tokio_postgres::Error as PostgresError;

use crate::database::users::UserResponse;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SamiErrorWithData {
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for SamiErrorWithData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "field: {}, message:{}", self.field, self.message)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum SamiError {
    ValidationError(SamiErrorWithData),
    NotFoundError(SamiErrorWithData),
    UnexpectedError(SamiErrorWithData),
    #[display(fmt = "{:?}", field)]
    InternalError {
        field: String,
    },
    SqlError {
        field: String,
    },
    InvalidError(SamiErrorWithData),
    AccountCreationError(SamiErrorWithData),
}

impl ResponseError for SamiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match &*self {
            SamiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            SamiError::UnexpectedError(_) => StatusCode::GATEWAY_TIMEOUT,
            SamiError::InternalError { field: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            SamiError::SqlError { field: _ } => StatusCode::UNPROCESSABLE_ENTITY,
            SamiError::InvalidError(_) => StatusCode::EXPECTATION_FAILED,
            SamiError::AccountCreationError(_) => StatusCode::NOT_MODIFIED,
            SamiError::NotFoundError(_) => StatusCode::NOT_FOUND,
        }
    }
}

impl From<String> for SamiError {
    fn from(e: String) -> Self {
        SamiError::InternalError { field: e }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub field: Option<String>,
    pub message: Option<String>,
    pub success: bool,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {:?}", self)
    }
}

impl Into<ErrorResponse> for SamiError {
    fn into(self) -> ErrorResponse {
        match self {
            SamiError::ValidationError(e) => ErrorResponse {
                field: Some(e.field),
                message: Some(e.message),
                success: false,
            },
            SamiError::NotFoundError(e) => ErrorResponse {
                field: Some(e.field),
                message: Some(e.message),
                success: false,
            },
            SamiError::UnexpectedError(e) => ErrorResponse {
                field: Some(e.field),
                message: Some(e.message),
                success: false,
            },
            SamiError::InternalError { field } => ErrorResponse {
                field: None,
                message: Some(field),
                success: false,
            },
            SamiError::SqlError { field } => ErrorResponse {
                field: None,
                message: Some(field),
                success: false,
            },
            SamiError::InvalidError(e) => ErrorResponse {
                field: Some(e.field),
                message: Some(e.message),
                success: false,
            },
            SamiError::AccountCreationError(e) => ErrorResponse {
                field: Some(e.field),
                message: Some(e.message),
                success: false,
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
                success: true,
            },
            None => ErrorResponse {
                field: None,
                message: None,
                success: false,
            },
        }
    }
}

impl Into<UserResponse> for ErrorResponse {
    fn into(self) -> UserResponse {
        UserResponse {
            data: None,
            error: Some(self),
            success: false,
        }
    }
}
