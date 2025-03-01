use crate::context::BoscaContext;
use crate::models::content::slug::SlugType;
use crate::models::content::supplementary::MetadataSupplementary;
use crate::models::security::permission::PermissionAction;
use crate::util::security::get_principal_from_headers;
use async_graphql::Error;
use axum::body::Body;
use axum::extract::{Path, Query};
use axum::extract::State;
use http::{HeaderMap, StatusCode};
use log::error;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PathParams {
    slug: String
}

#[derive(Debug, Deserialize)]
pub struct Params {
    key: Option<String>,
}

async fn get_supplementary(
    ctx: &BoscaContext,
    params: &Params,
    metadata_id: &Uuid,
) -> Result<Option<MetadataSupplementary>, Error> {
    Ok(if params.key.is_none() {
        None
    } else {
        ctx.content
            .metadata
            .get_supplementary(metadata_id, params.key.as_ref().unwrap())
            .await?
    })
}

pub async fn slug(
    State(ctx): State<BoscaContext>,
    Path(PathParams { slug }): Path<PathParams>,
    Query(params): Query<Params>,
    headers: HeaderMap,
) -> Result<(HeaderMap, Body), (StatusCode, String)> {
    let principal = get_principal_from_headers(&ctx, &headers)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let slug = slug.split('.').next().unwrap();
    let slug = ctx
        .content
        .get_slug(slug)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
    if slug.is_none() {
        return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
    }
    let slug = slug.unwrap();
    if slug.slug_type != SlugType::Metadata {
        return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
    }
    let metadata = ctx
        .check_metadata_content_action_principal(&principal, &slug.id, PermissionAction::View)
        .await
        .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?;
    if metadata.deleted
        && !ctx
            .has_admin_account()
            .await
            .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?
    {
        return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
    }
    let supplementary = get_supplementary(&ctx, &params, &metadata.id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            )
        })?;
    let path = ctx
        .storage
        .get_metadata_path(&metadata, supplementary.as_ref().map(|s| s.key.clone()))
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            )
        })?;
    let buf = ctx.storage.get_buffer(&path).await.map_err(|e| {
        error!("Error getting buffer: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    let body = Body::from_stream(buf);
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        if supplementary.is_some() {
            supplementary.unwrap().content_type.parse().map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            })?
        } else {
            let content_type = metadata.content_type.parse().map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            })?;
            if content_type == "audio/mpeg" && metadata.name.ends_with(".mp3") {
                "audio/mp3".parse().unwrap()
            } else {
                content_type
            }
        },
    );
    Ok((headers, body))
}
