use crate::context::{BoscaContext, PermissionCheck};
use crate::models::content::metadata_supplementary::MetadataSupplementary;
use crate::models::content::slug::{Slug, SlugType};
use crate::models::security::permission::PermissionAction;
use crate::util::security::get_principal_from_headers;
use async_graphql::Error;
use axum::body::Body;
use axum::extract::State;
use axum::extract::{Path, Query};
use http::{header, HeaderMap, HeaderValue, StatusCode};
use log::error;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PathParams {
    slug: String,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    supplementary_id: Option<String>,
    key: Option<String>,
    download: Option<bool>,
}

#[tracing::instrument(skip(ctx, params))]
async fn get_supplementary(
    ctx: &BoscaContext,
    metadata_id: &Uuid,
    params: &Params,
) -> Result<Option<MetadataSupplementary>, Error> {
    Ok(if let Some(key) = &params.key {
        ctx.content
            .metadata_supplementary
            .get_supplementary_by_key(metadata_id, key)
            .await?
    } else if let Some(supplementary_id) = &params.supplementary_id {
        let supplementary_id = Uuid::parse_str(supplementary_id.as_str())?;
        ctx.content
            .metadata_supplementary
            .get_supplementary(&supplementary_id)
            .await?
    } else {
        None
    })
}

#[tracing::instrument(skip(ctx, slug, params, headers))]
pub async fn slug(
    State(ctx): State<BoscaContext>,
    Path(PathParams { slug }): Path<PathParams>,
    Query(params): Query<Params>,
    headers: HeaderMap,
) -> Result<(HeaderMap, Body), (StatusCode, String)> {
    let principal = get_principal_from_headers(&ctx, &headers)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let principal_groups = ctx
        .security
        .get_principal_groups(&principal.id)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let slug = slug.split('.').next().unwrap();
    let mut slug_content = ctx
        .content
        .get_slug(slug)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
    if slug_content.is_none() {
        if let Ok(id) = Uuid::parse_str(slug) {
            slug_content = Some(Slug {
                id,
                slug_type: SlugType::Metadata,
            })
        } else {
            return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
        }
    }
    let slug = slug_content.unwrap();
    if slug.slug_type != SlugType::Metadata {
        return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
    }
    let check = PermissionCheck::new_with_principal_and_metadata_id(
        principal,
        principal_groups,
        slug.id,
        PermissionAction::View,
    );
    let metadata = ctx
        .metadata_permission_check(check)
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
    let supplementary = get_supplementary(&ctx, &metadata.id, &params)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            )
        })?;
    let path = ctx
        .storage
        .get_metadata_path(&metadata, supplementary.as_ref().map(|s| s.id))
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            )
        })?;

    let mut headers = HeaderMap::new();
    headers.insert(header::ACCEPT_RANGES, HeaderValue::from_static("bytes"));
    if let Some(range_header) = headers.get(header::RANGE) {
        if let Ok(range) = crate::metadata_files::get_range_header(range_header) {
            let (buf, size, range) = ctx
                .storage
                .get_buffer_range(&path, range)
                .await
                .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid Range".to_string()))?;
            let content_length = range.end - range.start + 1;
            headers.insert(header::CONTENT_LENGTH, HeaderValue::from(content_length));
            headers.insert(
                header::CONTENT_RANGE,
                HeaderValue::from_str(&format!("bytes {}-{}/{}", range.start, range.end, size))
                    .map_err(|_| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to create Content-Range header".to_string(),
                        )
                    })?,
            );
            let body = Body::from_stream(buf);
            return Ok((headers, body));
        }
    }

    let (buf, size) = ctx.storage.get_buffer(&path).await.map_err(|e| {
        error!("Error getting buffer: {e}");
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    let body = Body::from_stream(buf);

    if params.download.unwrap_or(false) {
        let mut filename = metadata.name.to_string();
        if metadata.content_type.starts_with("image/")
            || metadata.content_type.starts_with("video/")
            || metadata.content_type.starts_with("audio/")
        {
            let parts = metadata.content_type.split("/");
            filename = format!("{filename}.{}", parts.last().unwrap_or(""));
        }
        let disposition = format!("attachment; filename=\"{filename}\"");
        headers.insert(
            header::CONTENT_DISPOSITION,
            disposition.parse().map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            })?,
        );
    }

    headers.insert(header::CONTENT_LENGTH, size.into());
    headers.insert(
        header::CONTENT_TYPE,
        if let Some(supplementary) = &supplementary {
            supplementary.content_type.parse().map_err(|_| {
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
