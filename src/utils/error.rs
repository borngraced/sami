use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum SamiError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
    #[display(fmt = "Password doesn't match with account, try again")]
    PasswordError,
    #[display(fmt = "Account Not found")]
    AccountNotFoundError,
    #[display(fmt = "Unexpectesd Error occured")]
    UnexpectedError { field: String },
    #[display(fmt = "Error while communicating with the databse")]
    DatabaseError,
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
    #[display(fmt = "Invlid email address: {}", field)]
    InvalidEmail { field: String },
    #[display(fmt = "{}", field)]
    AccountCreationError { field: String },
}

impl ResponseError for SamiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match &*self {
            SamiError::ValidationError { field: _ } => StatusCode::BAD_REQUEST,
            SamiError::PasswordError => StatusCode::UNAUTHORIZED,
            SamiError::UnexpectedError { field: _ } => StatusCode::GATEWAY_TIMEOUT,
            SamiError::DatabaseError => StatusCode::REQUEST_TIMEOUT,
            SamiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            SamiError::InvalidEmail { field: _ } => StatusCode::EXPECTATION_FAILED,
            SamiError::AccountCreationError { field: _ } => StatusCode::NOT_MODIFIED,
            SamiError::AccountNotFoundError => todo!(),
        }
    }
}
