use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use tokio_postgres::{
    types::{FromSql, ToSql},
    Client,
};

use crate::utils::{
    error::{ErrorResponse, SamiError},
    validator::{encode_password, validate_email, verify_password},
};

use super::statements::{CREATE_USER_TABLE, GET_SINGLE_USER, INSERT_USER};

type SamiWebResponse<T> = Result<T, ErrorResponse>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserResponse {
    pub data: Option<UserResponseData>,
    pub error: Option<ErrorResponse>,
    pub success: bool,
}

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserResponseData {
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

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::HeadBoy => format!("{:?}", Self::HeadBoy),
            Role::NoRole => format!("{:?}", Self::NoRole),
        }
    }
}

impl Default for Role {
    fn default() -> Self {
        Self::HeadBoy
    }
}

impl From<String> for Role {
    fn from(e: String) -> Self {
        match &e.to_lowercase().as_str() {
            &"headboy" => Self::HeadBoy,
            _ => Self::NoRole,
        }
    }
}

impl FromSql<'_> for Role {
    fn from_sql<'a>(
        ty: &tokio_postgres::types::Type,
        _raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        match ty.kind() {
            tokio_postgres::types::Kind::Simple => Ok(Self::HeadBoy),
            tokio_postgres::types::Kind::Enum(_) => Ok(Self::HeadBoy),
            _ => Ok(Self::NoRole),
        }
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        match ty.kind() {
            tokio_postgres::types::Kind::Simple => true,
            tokio_postgres::types::Kind::Enum(_) => true,
            _ => false,
        }
    }
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

pub async fn create_table(client: &Client) -> Result<u64, SamiError> {
    client
        .execute(CREATE_USER_TABLE, &[])
        .await
        .map_err(|e| SamiError::InternalError {
            field: e.to_string(),
        })
}

pub async fn login(client: &Client, user: &UserLoginRequest) -> SamiWebResponse<UserResponse> {
    info!("Creating user {}../", user.email);

    let get_q = client
        .query_one(
            "SELECT password FROM users WHERE email = $1",
            &[&user.email.to_owned() as &(dyn ToSql + Sync)],
        )
        .await
        .map_err(|e| e.into())?;

    let parsed_hash = get_q.get("password");
    verify_password(user.password.to_owned(), parsed_hash).map_err(|e| {
        let re: ErrorResponse = e.into();
        re
    });

    let params = &[&user.email.to_owned() as &(dyn ToSql + Sync)];
    let row = client
        .query_one(GET_SINGLE_USER, params)
        .await
        .map_err(|e| e.into())?;

    Ok(UserResponse {
        data: Some(UserResponseData {
            uuid: row.get("uuid"),
            email: row.get("email"),
            username: row.get("username"),
            created_at: row.get("created_at"),
            role: row.get("role"),
        }),
        error: None,
        success: true,
    })
}

pub async fn add_new_user(client: &Client, user: &UserRequest) -> SamiWebResponse<UserResponse> {
    info!("Creating user {}../", user.email);
    validate_email(&user.email).map_err(|e| e.into())?;
    let (password, role) = (
        encode_password(user.password.to_owned()).map_err(|e| e.into())?,
        Role::default(),
    );

    let params = vec![
        user.username.to_owned(),
        user.email.to_owned(),
        password,
        role.to_string(),
    ];
    let res = client
        .execute_raw(INSERT_USER, params)
        .await
        .map_err(|e| e.into())?;

    Ok(UserResponse {
        data: None,
        error: None,
        success: true,
    })
}

pub async fn get_single_user(
    client: &Client,
    user: &GetUserRequest,
) -> Result<UserResponse, ErrorResponse> {
    info!("Getting user data from DB {}../", user.email);
    let params = &[&user.email.to_owned() as &(dyn ToSql + Sync)];
    let row = client
        .query_one(GET_SINGLE_USER, params)
        .await
        .map_err(|e| e.into())?;

    Ok(UserResponse {
        data: Some(UserResponseData {
            uuid: row.get("uuid"),
            email: row.get("email"),
            username: row.get("username"),
            created_at: row.get("created_at"),
            role: row.get("role"),
        }),
        error: None,
        success: true,
    })
}
