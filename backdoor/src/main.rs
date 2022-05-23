mod common;
mod database;
mod library;
mod routes;

#[macro_use]
extern crate log;

use crate::common::{error::SamiError, responder::SamiWebResponse};
use crate::library::validator;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use database::SamiCtx;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> SamiWebResponse<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    dotenv().ok();

    let db = SamiCtx::ready().await?;
    db.init_db().await.map_err(|e| e.into())?;
    let data = web::Data::new(db.client);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("127.0.0.1:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(HttpAuthentication::bearer(validator))
            .wrap(Logger::new("%r %s %T"))
            .app_data(data.clone())
            .service(routes::auth)
            .service(routes::new_user)
            .service(routes::get_user)
            .service(routes::new_article)
            .service(routes::get_one_article)
            .service(routes::get_all_article)
            .service(routes::update_one_article)
            .service(routes::delete_one_article)
    })
    .bind(("127.0.0.1", 5050))
    .map_err(|e| {
        SamiError::InternalError {
            field: e.to_string(),
        }
        .into()
    })?
    .run()
    .await
    .map_err(|e| {
        SamiError::InternalError {
            field: e.to_string(),
        }
        .into()
    })
}
