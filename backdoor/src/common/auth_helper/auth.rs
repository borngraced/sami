use crate::common::{
    error::{ErrorResponse, SamiStatusCode},
    responder::SamiResponder,
};
use actix_session::Session;
use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.as_ref().clone())
        .unwrap_or_else(Default::default);
    match validate_token(credentials.token()).await {
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

pub async fn validate_token(token: &str) -> Result<bool, ErrorResponse> {
    let authority = std::env::var("AUTHORITY").map_err(|e| ErrorResponse {
        field: Some("Bearer Auth".to_string()),
        message: Some(e.to_string()),
        code: SamiStatusCode::AuthenticationFailed,
    })?;
    let jwks = fetch_jwks(&format!(
        "{}{}",
        authority.as_str(),
        ".well-known/jwks.json"
    ))
    .await
    .map_err(|e| ErrorResponse {
        field: Some("Bearer Auth".to_string()),
        message: Some(e.to_string()),
        code: SamiStatusCode::AuthenticationFailed,
    })?;

    let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];
    let kid = token_kid(&token).map_err(|e| ErrorResponse {
        field: Some("Bearer Auth".to_string()),
        message: Some(e.to_string()),
        code: SamiStatusCode::AuthenticationFailed,
    })?;

    match kid {
        Some(k) => {
            let jwk = jwks.find(&k).expect("Specified key not found in set");
            let res = validate(token, jwk, validations);
            Ok(res.is_ok())
        }
        None => {
            return Err(ErrorResponse {
                field: Some("Bearer Auth".to_string()),
                message: Some("Bearer authentication failed".to_string()),
                code: SamiStatusCode::Internal,
            })
        }
    }
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn std::error::Error>> {
    let res = reqwest::get(uri).await?;
    let val = res.json::<JWKS>().await?;
    return Ok(val);
}

pub fn verify_auth_token<T: Debug + Clone>(session: Session) -> Result<(), SamiResponder<T>> {
    info!("Authorizing user credentials");
    match session.get::<String>("user_email") {
        Ok(res) => {
            if let None = res {
                let err: SamiResponder<T> = ErrorResponse {
                    field: Some("Authorization".to_string()),
                    message: Some("You need to be sami to perform this action!".to_string()),
                    code: SamiStatusCode::AuthenticationFailed,
                }
                .into();
                return Err(err);
            }
        }
        Err(_) => {
            let err: SamiResponder<T> = ErrorResponse {
                field: Some("Authorization".to_string()),
                message: Some("You need to be sami to perform this action!".to_string()),
                code: SamiStatusCode::AuthenticationFailed,
            }
            .into();
            return Err(err);
        }
    };

    Ok(())
}
