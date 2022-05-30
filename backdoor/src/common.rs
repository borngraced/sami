use actix_web::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::types::FromSql;

#[path = "common/auth_helper/auth.rs"]
pub mod auth;
#[path = "common/error.rs"]
pub mod error;
#[path = "common/read_env.rs"]
pub mod read_env;
#[path = "common/responder.rs"]
pub mod responder;
#[path = "common/auth_helper/validator.rs"]
pub mod validator;

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArticleData {
    pub uuid: Option<i32>,
    pub title: String,
    pub content: String,
    pub summary: String,
    pub slug: String,
    pub likes: Option<i32>,
    pub published: bool,
    pub tags: Vec<String>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub author_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArticleDataReq {
    pub title: String,
    pub content: String,
    pub summary: String,
    pub slug: String,
    pub published: bool,
    pub tags: Vec<String>,
    pub author_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryData {
    pub uuid: i32,
    pub name: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub role: Role,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContentReq {
    pub content: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ArticleEditRequest {
    pub slug: String,
    pub field: String,
}
