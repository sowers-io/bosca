use crate::context::BoscaContext;
use crate::models::content::metadata::Metadata;
use crate::models::content::slug::SlugType;
use crate::models::security::permission::PermissionAction;
use crate::models::security::principal::Principal;
use crate::util::security::get_principal_from_headers;
use axum::body::Body;
use axum::extract::{Query, State};
use http::{header, HeaderMap, HeaderValue, StatusCode};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Params {
    id: Option<String>,
    slug: Option<String>,
    key: Option<String>,
    download: Option<bool>,
}

async fn get_image(
    ctx: &BoscaContext,
    principal: &Principal,
    groups: &Vec<Uuid>,
    metadata: Metadata,
    key: Option<String>,
    download: Option<bool>,
) -> Result<(HeaderMap, Body), (StatusCode, String)> {
    let (path, content_type) = if let Some(key) = key {
        if let Some(resized) = ctx
            .content
            .metadata_supplementary
            .get_supplementary_by_key(&metadata.id, &key)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error getting supplementary".to_owned(),
                )
            })?
        {
            if !ctx
                .content
                .metadata_permissions
                .has_supplementary_permission(&metadata, principal, groups, PermissionAction::View)
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Error getting supplementary".to_owned(),
                    )
                })?
            {
                let admin = ctx.security.get_administrators_group().await.map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Error getting supplementary".to_owned(),
                    )
                })?;
                if !groups.contains(&admin.id) {
                    return Err((StatusCode::UNAUTHORIZED, "Invalid Permissions".to_owned()));
                }
            }
            let path = ctx
                .storage
                .get_metadata_path(&metadata, Some(resized.id))
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error".to_owned(),
                    )
                })?;
            (path, resized.content_type)
        } else {
            let path = ctx
                .storage
                .get_metadata_path(&metadata, None)
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error".to_owned(),
                    )
                })?;
            (path, metadata.content_type)
        }
    } else {
        let path = ctx
            .storage
            .get_metadata_path(&metadata, None)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            })?;
        (path, metadata.content_type)
    };
    if !content_type.starts_with("image/") {
        return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
    }
    let buf = ctx.storage.get_buffer(&path).await;
    let (buf, size) = match buf {
        Ok(buf) => buf,
        Err(e) => {
            return match e {
                object_store::Error::NotFound { path: _, source: _ } => {
                    Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))
                }
                _ => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
        }
    };
    let body = Body::from_stream(buf);
    let mut headers = HeaderMap::new();

    if download.unwrap_or(false) {
        let mut filename = metadata.name;
        if content_type.starts_with("image/")
            || content_type.starts_with("video/")
            || content_type.starts_with("audio/")
        {
            let parts = content_type.split("/");
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

    headers.insert(header::CONTENT_LENGTH, HeaderValue::from(size));
    headers.insert(
        header::CONTENT_TYPE,
        content_type.parse().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            )
        })?,
    );

    Ok((headers, body))
}

#[tracing::instrument(skip(ctx, params, headers))]
pub async fn image(
    State(ctx): State<BoscaContext>,
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

    let metadata = if let Some(id) = params.id {
        let id = Uuid::parse_str(id.as_str())
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid ID".to_owned()))?;
        ctx.check_metadata_action_principal(
            &principal,
            &principal_groups,
            &id,
            PermissionAction::View,
        )
        .await
        .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?
    } else if let Some(slug) = params.slug {
        let Some(slug) = ctx.content.get_slug(&slug).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting slug".to_owned(),
            )
        })?
        else {
            return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
        };
        if slug.slug_type == SlugType::Metadata {
            let Some(metadata) = ctx.content.metadata.get(&slug.id).await.map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error getting metadata".to_owned(),
                )
            })?
            else {
                return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
            };
            metadata
        } else {
            return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
        }
    } else {
        return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
    };
    if metadata.deleted
        && !ctx
            .has_admin_account()
            .await
            .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?
    {
        return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
    }
    get_image(
        &ctx,
        &principal,
        &principal_groups,
        metadata,
        params.key,
        params.download,
    )
    .await
}
