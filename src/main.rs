mod common;
mod database;
mod library;

#[macro_use]
extern crate log;

use crate::library::validator;
use crate::{
    common::{
        error::SamiError,
        responder::{SamiWebResponse, UserData},
    },
    database::users::{add_new_user, get_single_user, login, UserLoginRequest, UserRequest},
};
use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use common::responder::SamiResponder;
use database::connect::SamiCtx;
use dotenv::dotenv;
use tokio_postgres::Client;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/auth")]
async fn auth(client: web::Data<Client>, req: web::Json<UserLoginRequest>) -> impl Responder {
    let client = client.as_ref();
    let user = UserLoginRequest {
        email: req.email.to_string(),
        password: req.password.to_string(),
    };

    login(&client, &user)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| {
            let err: SamiResponder<UserData> = err.into();
            err
        })
}

#[post("/user")]
async fn new_user(client: web::Data<Client>, req: web::Json<UserRequest>) -> impl Responder {
    let client = client.as_ref();
    add_new_user(&client, &req)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| {
            let err: SamiResponder<UserData> = err.into();
            err
        })
}

#[get("/user/{uuid}")]
async fn get_user(client: web::Data<Client>, uuid: web::Path<(i32,)>) -> impl Responder {
    let client = client.as_ref();
    let uuid = uuid.into_inner().0;

    get_single_user(&client, uuid)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| {
            let err: SamiResponder<UserData> = err.into();
            err
        })
}

#[actix_web::main]
async fn main() -> SamiWebResponse<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    dotenv().ok();

    let db = SamiCtx::ready().await?;
    let _ = db.init_db().await;
    let data = web::Data::new(db.client);

    HttpServer::new(move || {
        App::new()
            .wrap(HttpAuthentication::bearer(validator))
            .wrap(Logger::new("%r %s %T"))
            .app_data(data.clone())
            .service(auth)
            .service(hello)
            .service(new_user)
            .service(get_user)
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
