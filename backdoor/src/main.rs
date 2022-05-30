mod common;
mod database;
mod routes;

#[macro_use]
extern crate log;

use crate::common::responder::SamiWebResponse;
use actix_cors::Cors;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::http::header;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use common::auth::validator;
use common::error::ErrorResponse;
use database::SamiCtx;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> SamiWebResponse<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    dotenv().ok();

    let db = SamiCtx::ready().await?;
    db.init_db().await?;
    let data = web::Data::new(db.client);
    let secret_key = Key::generate();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("127.0.0.1:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        //.wrap(IdentityService::new(policy))
        App::new()
            .wrap(cors)
            .wrap(HttpAuthentication::bearer(validator))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.to_owned(),
            ))
            .wrap(Logger::new("%r %s %T"))
            .app_data(data.clone())
            .service(routes::auth)
            .service(routes::new_user)
            .service(routes::get_user)
            .service(routes::logout)
            .service(routes::new_article)
            .service(routes::get_one_article)
            .service(routes::get_all_article)
            .service(routes::update_one_article)
            .service(routes::delete_one_article)
    })
    .bind(("127.0.0.1", 5500))
    .map_err(|e| ErrorResponse {
        field: None,
        message: Some(e.to_string()),
        code: common::error::SamiStatusCode::Internal,
    })?
    .run()
    .await
    .map_err(|e| ErrorResponse {
        field: None,
        message: Some(e.to_string()),
        code: common::error::SamiStatusCode::Internal,
    })
}
