use actix_session::Session;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use tokio_postgres::Client;

use crate::{
    common::{
        auth::verify_auth_token, responder::SamiResponder, ArticleData, ArticleDataReq,
        ArticleEditRequest, ContentReq, UserData,
    },
    database::{
        article::{
            add_new_article_to_db, delete_article_from_db, get_article_from_db,
            get_articles_from_db, update_article_to_db,
        },
        users::{add_new_user, get_single_user, login, UserLoginRequest, UserRequest},
    },
};

// START USER API CALLS
#[post("/auth/")]
pub async fn auth(
    client: web::Data<Client>,
    req: web::Json<UserLoginRequest>,
    session: Session,
) -> impl Responder {
    let client = client.as_ref();
    let user = UserLoginRequest {
        email: req.email.to_string(),
        password: req.password.to_string(),
    };

    login(&client, &user, session)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| {
            let err: SamiResponder<UserData> = err.into();
            err
        })
}
// START USER API CALLS
#[post("/logout/")]
pub async fn logout(session: Session) -> impl Responder {
    session.clear();
    HttpResponse::Ok()
}

#[post("/user/")]
pub async fn new_user(
    client: web::Data<Client>,
    req: web::Json<UserRequest>,
    session: Session,
) -> impl Responder {
    let client = client.as_ref();
    add_new_user(&client, &req, session)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| {
            let err: SamiResponder<UserData> = err.into();
            err
        })
}

#[get("/user/{uuid}/")]
pub async fn get_user(
    client: web::Data<Client>,
    uuid: web::Path<(i32,)>,
    session: Session,
) -> impl Responder {
    verify_auth_token::<UserData>(session)?;
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
#[post("/article/")]
pub async fn new_article(
    client: web::Data<Client>,
    req: web::Json<ArticleDataReq>,
    session: Session,
) -> impl Responder {
    verify_auth_token::<ArticleDataReq>(session)?;

    let client = client.as_ref();
    add_new_article_to_db(&client, &req)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| {
            let err: SamiResponder<ArticleDataReq> = err.into();
            err
        })
}

#[get("/article/{slug}/")]
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
            let err: SamiResponder<ArticleData> = err.into();
            err
        })
}

#[get("/articles/")]
pub async fn get_all_article(client: web::Data<Client>) -> impl Responder {
    let client = client.as_ref();

    get_articles_from_db(&client)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| {
            let err: SamiResponder<Vec<ArticleData>> = err.into();
            err
        })
}

#[put("/article/")]
pub async fn update_one_article(
    client: web::Data<Client>,
    req: web::Json<ContentReq>,
    path: web::Query<ArticleEditRequest>,
    session: Session,
) -> impl Responder {
    verify_auth_token::<ArticleData>(session)?;

    let client = client.as_ref();
    let path = path.into_inner();
    let (slug, field) = (&path.slug, &path.field);
    println!("{}:{}", slug, field);
    update_article_to_db(&client, &req, slug.to_owned(), field.to_owned())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| {
            let err: SamiResponder<ArticleData> = err.into();
            err
        })
}

#[delete("/article/{slug}/")]
pub async fn delete_one_article(
    client: web::Data<Client>,
    slug: web::Path<(String,)>,
    session: Session,
) -> impl Responder {
    verify_auth_token::<ArticleData>(session)?;

    let client = client.as_ref();
    let slug = slug.into_inner().0;

    delete_article_from_db(&client, slug.as_str())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| {
            let err: SamiResponder<ArticleData> = err.into();
            err
        })
}
