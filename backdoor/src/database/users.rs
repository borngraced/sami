use derive_more::Display;
use serde::{Deserialize, Serialize};
use tokio_postgres::{
    types::{FromSql, ToSql},
    Client,
};

use crate::common::{
    error::{ErrorResponse, SamiError, SamiStatusCode},
    responder::{SamiResponder, SamiWebResponse},
    validator::{
        encode_password, validate_email, validate_password, validate_username, verify_password,
    },
    Role, UserData,
};

use super::statements::{GET_SINGLE_USER, INSERT_USER};

#[derive(Debug, Clone, Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetUserRequest {
    pub email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Display)]
#[display(fmt = "my error: {}", message)]
pub struct LoginErrResponse {
    pub error: bool,
    pub message: String,
}

// impl Pos for

pub async fn login(
    client: &Client,
    user: &UserLoginRequest,
) -> SamiWebResponse<SamiResponder<UserData>> {
    info!("Creating user {}../", user.email);

    let get_password = client
        .query_one(
            "SELECT password FROM users WHERE email = $1",
            &[&user.email.to_owned() as &(dyn ToSql + Sync)],
        )
        .await
        .map_err(|e| e.into())?;

    let parsed_hash: String = get_password.get("password");

    verify_password(&user.password, &parsed_hash).map_err(|e| {
        let re: ErrorResponse = e.into();
        re
    })?;

    let params = &[&user.email.to_owned() as &(dyn ToSql + Sync)];
    let row = client
        .query_one(GET_SINGLE_USER, params)
        .await
        .map_err(|e| e.into())?;

    Ok(SamiResponder {
        data: Some(UserData {
            uuid: row.get("uuid"),
            email: row.get("email"),
            username: row.get("username"),
            created_at: row.get("created_at"),
            role: row.get("role"),
        }),
        error: None,
        success: true,
        code: SamiStatusCode::OK,
    })
}

pub async fn add_new_user(
    client: &Client,
    user: &UserRequest,
) -> SamiWebResponse<SamiResponder<UserData>> {
    info!("Creating user {}../", user.email);

    validate_email(&user.email).map_err(|e| e.into())?;
    validate_username(&user.username).map_err(|e| e.into())?;
    validate_password(&user.password).map_err(|e| e.into())?;

    let (password, role) = (
        encode_password(user.password.to_owned()).map_err(|e| e.into())?,
        Role::default(),
    );

    let params = vec![
        user.username.to_owned().to_lowercase(),
        user.email.to_owned().to_lowercase(),
        password,
        role.to_string(),
    ];
    let _ = client
        .execute_raw(INSERT_USER, params)
        .await
        .map_err(|err| err.into())?;

    Ok(SamiResponder {
        data: None,
        error: None,
        success: true,
        code: SamiStatusCode::OK,
    })
}

pub async fn get_single_user(
    client: &Client,
    uuid: i32,
) -> Result<SamiResponder<UserData>, ErrorResponse> {
    info!("Getting user data from DB uuid:{}../", uuid);
    let params = &[&uuid.to_owned() as &(dyn ToSql + Sync)];
    let row = client
        .query_one(GET_SINGLE_USER, params)
        .await
        .map_err(|e| e.into())?;

    Ok(SamiResponder {
        data: Some(UserData {
            uuid: row.get("uuid"),
            email: row.get("email"),
            username: row.get("username"),
            created_at: row.get("created_at"),
            role: row.get("role"),
        }),
        error: None,
        success: true,
        code: SamiStatusCode::OK,
    })
}
