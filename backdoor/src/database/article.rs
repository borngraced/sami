use tokio_postgres::{types::ToSql, Client, ToStatement};

use crate::common::{
    error::{ErrorResponse, SamiStatusCode},
    responder::{SamiResponder, SamiWebResponse},
    ArticleData, ArticleDataReq, ArticleUpdateData,
};

use super::statements::{
    GET_ALL_ARTICLE, GET_SINGLE_ARTICLE, INSERT_ARTICLE, UPDATE_SINGLE_ARTICLE_CONTENT,
    UPDATE_SINGLE_ARTICLE_DESC, UPDATE_SINGLE_ARTICLE_LIKES, UPDATE_SINGLE_ARTICLE_PUBLISHED,
    UPDATE_SINGLE_ARTICLE_TITLE,
};

pub async fn add_new_article_to_db(
    client: &Client,
    article_data: &ArticleDataReq,
) -> SamiWebResponse<SamiResponder<String>> {
    println!("{:?}", article_data);
    let params = &[
        &article_data.title.to_owned() as &(dyn ToSql + Sync),
        &article_data.content.to_owned() as &(dyn ToSql + Sync),
        &article_data.summary.to_owned() as &(dyn ToSql + Sync),
        &article_data.slug.to_owned() as &(dyn ToSql + Sync),
        &article_data.published.to_owned() as &(dyn ToSql + Sync),
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
            updated_at: row.get("updated_at"),
            created_at: row.get("created_at"),
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

    client
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

pub async fn update_article_to_db(
    client: &Client,
    data: &ArticleUpdateData,
) -> SamiWebResponse<SamiResponder<String>> {
    
    match data.field {
        crate::common::ArticleUpdateDataEnum::Title => {
            update_helper(client, data, UPDATE_SINGLE_ARTICLE_TITLE).await
        }
        crate::common::ArticleUpdateDataEnum::Content => {
            update_helper(client, data, UPDATE_SINGLE_ARTICLE_CONTENT).await
        }
        crate::common::ArticleUpdateDataEnum::Summary => {
            update_helper(client, data, UPDATE_SINGLE_ARTICLE_DESC).await
        }
        crate::common::ArticleUpdateDataEnum::Likes => {
            update_helper(client, data, UPDATE_SINGLE_ARTICLE_LIKES).await
        }
        crate::common::ArticleUpdateDataEnum::Published => {
            update_helper(client, data, UPDATE_SINGLE_ARTICLE_PUBLISHED).await
        }
        crate::common::ArticleUpdateDataEnum::None => Err(ErrorResponse {
            field: Some(
                "one of the fields must be set (title, content, likes, summary, published)"
                    .to_string(),
            ),
            message: Some("Please set the field you'd like to update..i.e title".to_string()),
            code: SamiStatusCode::ExpectationFailed,
        }),
    }
}

async fn update_helper<T: ToStatement + ?Sized>(
    client: &Client,
    data: &ArticleUpdateData,
    statement: &T,
) -> SamiWebResponse<SamiResponder<String>> {
    let params = &[
        &data.slug.to_owned() as &(dyn ToSql + Sync),
        &data.content.to_owned() as &(dyn ToSql + Sync),
    ];

    client
        .execute(statement, params)
        .await
        .map_err(|err| err.into())?;

    Ok(SamiResponder {
        data: Some("Action Successful".to_string()),
        error: None,
        success: true,
        code: SamiStatusCode::OK,
    })
}
