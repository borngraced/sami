mod database;
mod utils;

#[macro_use]
extern crate log;

use actix_web::{
    get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
    Result as ActixWebResult,
};
use database::connect::SamiCtx;
use tokio_postgres::Client;

use crate::{
    database::users::{
        add_new_user, get_single_user, login, GetUserRequest, UserLoginRequest, UserRequest,
    },
    utils::error::SamiError,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/auth")]
async fn auth(
    client: web::Data<Client>,
    req: web::Json<UserLoginRequest>,
) -> ActixWebResult<impl Responder, SamiError> {
    let client = client.as_ref();
    let user = UserLoginRequest {
        email: req.email.to_string(),
        password: req.password.to_string(),
    };
    let login = login(&client, &user).await;
    match login {
        Ok(t) => Ok(web::Json(t)),
        Err(e) => return Err(e.to_owned()),
    }
}

#[post("/user")]
async fn new_user(
    client: web::Data<Client>,
    req: web::Json<UserRequest>,
) -> ActixWebResult<impl Responder, SamiError> {
    let client = client.as_ref();
    let user = add_new_user(&client, &req).await;
    match user {
        Ok(_) => Ok(format!("Account Creation Successful")),
        Err(e) => return Err(e),
    }
}

#[get("/user")]
async fn get_user(
    client: web::Data<Client>,
    req: web::Json<GetUserRequest>,
) -> ActixWebResult<impl Responder, SamiError> {
    let client = client.as_ref();
    let user = GetUserRequest {
        email: req.email.to_string(),
    };
    let user = get_single_user(&client, &user).await;
    match user {
        Ok(t) => Ok(web::Json(t)),
        Err(e) => Err(e.to_owned()),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let db = SamiCtx::ready().await;
    let _ = db.init_db().await;
    let data = web::Data::new(db.client);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%r %s %T"))
            .app_data(data.clone())
            .service(auth)
            .service(hello)
            .service(new_user)
            .service(get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
