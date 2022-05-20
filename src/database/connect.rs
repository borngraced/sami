use tokio_postgres::{Client, Error, NoTls};

use super::users::create_table;

pub struct SamiCtx {
    pub client: Client,
}

impl SamiCtx {
    pub async fn ready() -> Self {
        let (client, connection) = tokio_postgres::connect(
            "host=localhost port=5432 user=sami password=SamopE! dbname=sami_rs",
            NoTls,
        )
        .await
        .expect("connection error");

        info!("Connection to DB");
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                info!("connection error: {}", e);
            };
        });
        Self { client }
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
