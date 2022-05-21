use tokio_postgres::{Client, Error, NoTls};

use crate::utils::{error::SamiError, read_env::db_from_env};

use super::users::create_table;

pub struct SamiCtx {
    pub client: Client,
}

impl SamiCtx {
    pub async fn ready() -> Result<Self, SamiError> {
        let config = db_from_env("DB").map_err(|e| e)?;
        let (client, connection) = tokio_postgres::connect(config.as_str(), NoTls)
            .await
            .map_err(|e| SamiError::InternalError {
                field: e.to_string(),
            })?;

        tokio::spawn(async move {
            connection.await.map_err(|e| SamiError::InternalError {
                field: e.to_string(),
            })
        });
        Ok(Self { client })
    }

    pub async fn init_db(&self) -> Result<u64, Error> {
        info!("Starting DB Migration -> Creating Table USERS");
        Ok(create_table(&self.client).await.unwrap())
    }

    // pub async fn start(self) -> Self {
    //     eprintln!("connection successful");
    //     tokio::spawn(async move {
    //         if let Err(e) = self.connection.into_inner().unwrap().await {
    //             eprintln!("connection error: {}", e);
    //         }
    //     });

    //     Self {
    //         client: self.client,
    //         connection: self.connection,
    //     }
    // }
}

// pub trait DBHelper {
//     fn start(connection: Connection<Socket, NoTlsStream>);
// }

// impl DBHelper for SamiCtx {

// }
