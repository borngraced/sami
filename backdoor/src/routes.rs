use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use tokio_postgres::Client;

use crate::{
    common::{responder::SamiResponder, ArticleData, ArticleDataReq, ArticleUpdateData, UserData},
    database::{
        article::{
            add_new_article_to_db, delete_article_from_db, get_article_from_db,
            get_articles_from_db, update_article_to_db,
        },
        users::{add_new_user, get_single_user, login, UserLoginRequest, UserRequest},
    },
};

// START USER API CALLS
#[get("/auth")]
pub async fn auth(client: web::Data<Client>, req: web::Json<UserLoginRequest>) -> impl Responder {
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
pub async fn new_user(client: web::Data<Client>, req: web::Json<UserRequest>) -> impl Responder {
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
pub async fn get_user(client: web::Data<Client>, uuid: web::Path<(i32,)>) -> impl Responder {
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

// START ARTICLE API CALLS
#[post("/article")]
pub async fn new_article(
    client: web::Data<Client>,
    req: web::Json<ArticleDataReq>,
) -> impl Responder {
    let client = client.as_ref();
    add_new_article_to_db(&client, &req)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| {
            let err: SamiResponder<ArticleDataReq> = err.into();
            err
        })
}

#[get("/article/{slug}")]
pub async fn get_one_article(
    client: web::Data<Client>,
    slug: web::Path<(String,)>,
) -> impl Responder {
    let client = client.as_ref();
    let slug = slug.into_inner().0;

    get_article_from_db(&client, slug.as_str())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| {
            let err: SamiResponder<UserData> = err.into();
            err
        })
}

#[get("/articles")]
pub async fn get_all_article(client: web::Data<Client>) -> impl Responder {
    let client = client.as_ref();

    get_articles_from_db(&client)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| {
            let err: SamiResponder<UserData> = err.into();
            err
        })
}

#[put("/article")]
pub async fn update_one_article(
    client: web::Data<Client>,
    req: web::Json<ArticleUpdateData>,
) -> impl Responder {
    let client = client.as_ref();

    update_article_to_db(&client, &req)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| {
            let err: SamiResponder<UserData> = err.into();
            err
        })
}

#[delete("/article/{slug}")]
pub async fn delete_one_article(
    client: web::Data<Client>,
    slug: web::Path<(String,)>,
) -> impl Responder {
    let client = client.as_ref();
    let slug = slug.into_inner().0;

    delete_article_from_db(&client, slug.as_str())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| {
            let err: SamiResponder<UserData> = err.into();
            err
        })
}
