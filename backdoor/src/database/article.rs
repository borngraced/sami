use serde::Serialize;
use serde_json::Value;
use tokio_postgres::{types::ToSql, Client, ToStatement};

use crate::common::{
    error::{ErrorResponse, SamiStatusCode},
    responder::{SamiResponder, SamiWebResponse},
    ArticleData, ArticleDataReq, ContentReq,
};

use super::statements::{
    DELETE_SINGLE_ARTICLE, GET_ALL_ARTICLE, GET_SINGLE_ARTICLE, INSERT_ARTICLE,
    UPDATE_SINGLE_ARTICLE_CONTENT, UPDATE_SINGLE_ARTICLE_DESC, UPDATE_SINGLE_ARTICLE_LIKES,
    UPDATE_SINGLE_ARTICLE_PUBLISHED, UPDATE_SINGLE_ARTICLE_SLUG, UPDATE_SINGLE_ARTICLE_TITLE,
};

pub async fn add_new_article_to_db(
    client: &Client,
    article_data: &ArticleDataReq,
) -> SamiWebResponse<SamiResponder<String>> {
    let params = &[
        &article_data.title.to_owned() as &(dyn ToSql + Sync),
        &article_data.content.to_owned() as &(dyn ToSql + Sync),
        &article_data.summary.to_owned() as &(dyn ToSql + Sync),
        &article_data.slug.to_owned() as &(dyn ToSql + Sync),
        &article_data.published.to_owned() as &(dyn ToSql + Sync),
        &article_data.tags.to_owned() as &(dyn ToSql + Sync),
        &article_data.author_id.to_owned() as &(dyn ToSql + Sync),
    ];
    let _ = client
        .execute(INSERT_ARTICLE, params)
        .await
        .map_err(|err| err.into())?;

    Ok(SamiResponder {
        data: Some("Action Successful".to_string()),
        error: None,
        success: true,
        code: SamiStatusCode::OK,
    })
}

pub async fn get_article_from_db(
    client: &Client,
    slug: &str,
) -> SamiWebResponse<SamiResponder<ArticleData>> {
    let params = &[&slug.to_owned() as &(dyn ToSql + Sync)];

    let row = client
        .query_one(GET_SINGLE_ARTICLE, params)
        .await
        .map_err(|err| err.into())?;
    let article_data = ArticleData {
        uuid: row.get("uuid"),
        title: row.get("title"),
        content: row.get("content"),
        summary: row.get("summary"),
        slug: row.get("slug"),
        likes: row.get("likes"),
        published: row.get("published"),
        tags: row.get("tags"),
        author_id: row.get("author_id"),
        updated_at: row.get("updated_at"),
        created_at: row.get("created_at"),
    };
    Ok(SamiResponder {
        data: Some(article_data),
        error: None,
        success: true,
        code: SamiStatusCode::OK,
    })
}

pub async fn get_articles_from_db(
    client: &Client,
) -> SamiWebResponse<SamiResponder<Vec<ArticleData>>> {
    let mut res = vec![];
    let rows = client
        .query(GET_ALL_ARTICLE, &[])
        .await
        .map_err(|err| err.into())?;

    for row in rows {
        res.push(ArticleData {
            uuid: row.get("uuid"),
            title: row.get("title"),
            content: row.get("content"),
            summary: row.get("summary"),
            slug: row.get("slug"),
            likes: row.get("likes"),
            published: row.get("published"),
            tags: row.get("tags"),
            updated_at: row.get("updated_at"),
            created_at: row.get("created_at"),
            author_id: row.get("author_id"),
        })
    }
    Ok(SamiResponder {
        data: Some(res),
        error: None,
        success: true,
        code: SamiStatusCode::OK,
    })
}

pub async fn delete_article_from_db(
    client: &Client,
    slug: &str,
) -> SamiWebResponse<SamiResponder<String>> {
    let params = &[&slug.to_owned() as &(dyn ToSql + Sync)];

    dbg!("Checking if article with slug exist in the database");
    if let Err(_) = client
        .query_one("SELECT slug FROM article where slug = $1", &[&slug])
        .await
    {
        return Err(ErrorResponse {
            field: Some("slug".to_string()),
            message: Some("You can't delete not found article!".to_string()),
            code: SamiStatusCode::ExpectationFailed,
        });
    }

    dbg!("Deleting article");
    client
        .execute(DELETE_SINGLE_ARTICLE, params)
        .await
        .map_err(|err| err.into())?;

    Ok(SamiResponder {
        data: Some("Action Successful".to_string()),
        error: None,
        success: true,
        code: SamiStatusCode::OK,
    })
}

pub async fn update_article_to_db(
    client: &Client,
    value: &ContentReq,
    slug: String,
    field: String,
) -> SamiWebResponse<SamiResponder<String>> {
    match &value.content {
        Value::Bool(e) => process_entity(client, slug, field, e).await,
        Value::Number(e) => process_entity(client, slug, field, e.as_i64().unwrap() as i32).await,
        Value::String(e) => process_entity(client, slug, field, e).await,
        _ => todo!(),
    }
}

async fn process_entity<Z: Clone>(
    client: &Client,
    slug: String,
    field: String,
    e: Z,
) -> SamiWebResponse<SamiResponder<String>>
where
    <Z as ToOwned>::Owned: ToSql + Sync + Serialize,
{
    match field.to_string().replace('"', "").as_str() {
        "title" => process_executor(client, &slug, e, UPDATE_SINGLE_ARTICLE_TITLE).await,
        "slug" => process_executor(client, &slug, e, UPDATE_SINGLE_ARTICLE_SLUG).await,
        "content" => process_executor(client, &slug, e, UPDATE_SINGLE_ARTICLE_CONTENT).await,
        "summary" => process_executor(client, &slug, e, UPDATE_SINGLE_ARTICLE_DESC).await,
        "likes" => process_executor(client, &slug, e, UPDATE_SINGLE_ARTICLE_LIKES).await,
        "published" => process_executor(client, &slug, e, UPDATE_SINGLE_ARTICLE_PUBLISHED).await,
        _ => Err(ErrorResponse {
            field: None,
            message: Some("Please set the field you'd like to update..i.e title".to_string()),
            code: SamiStatusCode::ExpectationFailed,
        }),
    }
}

async fn process_executor<T: ToStatement + ?Sized, Z: Clone>(
    client: &Client,
    slug: &String,
    e: Z,
    statement: &T,
) -> SamiWebResponse<SamiResponder<String>>
where
    <Z as ToOwned>::Owned: ToSql + Sync + Serialize,
{
    dbg!("Checking if article with slug exist in the database");

    if let Err(_) = client
        .query_one("SELECT slug FROM article where slug = $1", &[&slug])
        .await
    {
        return Err(ErrorResponse {
            field: Some("slug".to_string()),
            message: Some("No Post found for the provided slug".to_string()),
            code: SamiStatusCode::ExpectationFailed,
        });
    }

    dbg!("Updating field with data");

    client
        .execute(statement, &[&slug.replace('"', "").replace("/", ""), &e])
        .await
        .map_err(|err| err.into())?;
    Ok(SamiResponder {
        data: Some("Action Successful".to_string()),
        error: None,
        success: true,
        code: SamiStatusCode::OK,
    })
}
