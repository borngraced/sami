use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::types::FromSql;

#[path = "common/error.rs"]
pub mod error;
#[path = "common/read_env.rs"]
pub mod read_env;
#[path = "common/responder.rs"]
pub mod responder;
#[path = "common/validator.rs"]
pub mod validator;

pub type SamiWebResponse<T> = Result<T, error::ErrorResponse>;

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
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArticleDataReq {
    pub title: String,
    pub content: String,
    pub summary: String,
    pub slug: String,
    pub published: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArticleUpdateData {
    pub field: ArticleUpdateDataEnum,
    pub slug: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ArticleUpdateDataEnum {
    Title,
    Content,
    Summary,
    Published,
    Likes,
    None,
}

impl From<String> for ArticleUpdateDataEnum {
    fn from(e: String) -> Self {
        match e.to_lowercase().as_str() {
            "title" => ArticleUpdateDataEnum::Title,
            "content" => ArticleUpdateDataEnum::Content,
            "summary" => ArticleUpdateDataEnum::Summary,
            "published" => ArticleUpdateDataEnum::Published,
            "likes" => ArticleUpdateDataEnum::Likes,
            _ => ArticleUpdateDataEnum::None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryData {
    pub uuid: i32,
    pub name: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub role: Role,
}
