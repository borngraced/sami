#[path = "database/statements.rs"]
pub mod statements;
#[path = "database/users.rs"]
pub mod users;
pub mod article;

use tokio_postgres::{Client, NoTls};

use crate::common::{
    error::{ErrorResponse, SamiError, SamiStatusCode},
    read_env::data_from_env,
};

use self::statements::{
    CREATE_ARTCLE_TABLE, CREATE_CATEGORY_ARTICLE_TABLE, CREATE_CATEGORY_TABLE, CREATE_USER_TABLE,
};

pub struct SamiCtx {
    pub client: Client,
}

impl SamiCtx {
    pub async fn ready() -> Result<Self, ErrorResponse> {
        let config = data_from_env("DB").map_err(|e| e)?;
        let (client, connection) = tokio_postgres::connect(config.as_str(), NoTls)
            .await
            .map_err(|e| ErrorResponse {
                field: None,
                message: Some(e.to_string()),
                code: SamiStatusCode::Sql,
            })?;

        tokio::spawn(async move {
            connection.await.map_err(|e| ErrorResponse {
                field: None,
                message: Some(e.to_string()),
                code: SamiStatusCode::Sql,
            })
        });
        Ok(Self { client })
    }

    async fn create_migration(&self, statement: &str) -> Result<(), SamiError> {
        info!("Executing statement ->  {}", statement);

        self.client.execute(statement, &[]).await.map_err(|e| {
            println!("{}", e.to_string());
            SamiError::InternalError {
                field: e.to_string(),
            }
        })?;

        Ok(())
    }
    pub async fn init_db(&self) -> Result<(), SamiError> {
        info!("Starting DB Migration");
        let work_to_do = vec![
            CREATE_USER_TABLE,
            CREATE_ARTCLE_TABLE,
            CREATE_CATEGORY_TABLE,
            CREATE_CATEGORY_ARTICLE_TABLE,
        ];
        for st in work_to_do {
            self.create_migration(st).await?
        }

        Ok(())
    }
}
