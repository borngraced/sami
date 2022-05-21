mod database;
mod utils;

#[macro_use]
extern crate log;

use actix_web::{
    get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
    Result as ActixWebResult,
};
use database::connect::SamiCtx;
use dotenv::dotenv;
use tokio_postgres::Client;
use utils::error::SamiErrorWithData;

use crate::{
    database::users::{
        add_new_user, get_single_user, login, GetUserRequest, UserLoginRequest, UserRequest,
        UserResponse,
    },
    utils::error::{ErrorResponse, SamiError},
};

type SamiResult<T> = Result<T, SamiError>;

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
    let login_res = login(&client, &user).await.map_err(|e| {
        let err: UserResponse = e.into();
        web::Json(err)
    });
    Ok(web::Json(login_res))
}

#[post("/user")]
async fn new_user(client: web::Data<Client>, req: web::Json<UserRequest>) -> impl Responder {
    let client = client.as_ref();
    let res = match add_new_user(&client, &req).await {
        Ok(e) => e,
        Err(err) => err.into(),
    };
    web::Json(res)
}

#[get("/user")]
async fn get_user(client: web::Data<Client>, req: web::Json<GetUserRequest>) -> impl Responder {
    let client = client.as_ref();
    let user = GetUserRequest {
        email: req.email.to_string(),
    };
    let res = match get_single_user(&client, &user).await {
        Ok(res) => res,
        Err(err) => err.into(),
    };
    web::Json(res)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> SamiResult<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    dotenv().ok();

    let db = SamiCtx::ready().await?;
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
    .bind(("127.0.0.1", 5050))
    .map_err(|e| {
        SamiError::UnexpectedError(SamiErrorWithData {
            field: "port".to_string(),
            message: e.to_string(),
        })
    })?
    .run()
    .await
    .map_err(|e| {
        SamiError::UnexpectedError(SamiErrorWithData {
            field: "port".to_string(),
            message: e.to_string(),
        })
    })
}
