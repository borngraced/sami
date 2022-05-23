#[path = "library/auth.rs"]
pub mod auth;

use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.as_ref().clone())
        .unwrap_or_else(Default::default);
    match auth::validate_token(credentials.token()).await {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                let err = AuthenticationError::new(config);
                Err(Error::from(err))
            }
        }
        Err(_) => {
            let err = AuthenticationError::new(config);
            Err(Error::from(err))
        }
    }
}
