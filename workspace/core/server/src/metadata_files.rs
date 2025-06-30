use crate::context::BoscaContext;
use crate::models::content::metadata_supplementary::MetadataSupplementary;
use crate::models::security::permission::PermissionAction;
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::util::security::get_principal_from_headers;
use crate::util::upload::upload_field;
use crate::workflow::core_workflow_ids::METADATA_PROCESS;
use async_graphql::Error;
use axum::body::Body;
use axum::extract::{Multipart, Query};
use axum::extract::{Request, State};
use http::{HeaderMap, HeaderValue, StatusCode};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Params {
    id: Option<String>,
    supplementary_id: Option<String>,
    ready: Option<bool>,
    redirect: Option<String>,
}

#[tracing::instrument(skip(ctx, params))]
async fn get_supplementary(
    ctx: &BoscaContext,
    params: &Params,
) -> Result<Option<MetadataSupplementary>, Error> {
    if let Some(supplementary_id) = params.supplementary_id.as_ref() {
        let id = Uuid::parse_str(supplementary_id)?;
        return ctx
            .content
            .metadata_supplementary
            .get_supplementary(&id)
            .await;
    }
    Ok(None)
}

#[tracing::instrument(skip(ctx, params, headers, request))]
pub async fn metadata_download(
    State(ctx): State<BoscaContext>,
    Query(params): Query<Params>,
    headers: HeaderMap,
    request: Request<Body>,
) -> Result<(HeaderMap, Body), (StatusCode, String)> {
    let principal = get_principal_from_headers(&ctx, &headers)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let principal_groups = ctx.security.get_principal_groups(&principal.id).await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let id = params
        .id
        .as_ref()
        .map(|s| Uuid::parse_str(s.as_str()).unwrap())
        .unwrap_or_default();
    let url = format!("/files/metadata{}", request.uri().path_and_query().unwrap());
    let (metadata, supplementary) = if ctx.security.verify_signed_url(&url) {
        let metadata = ctx
            .content
            .metadata
            .get(&id)
            .await
            .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?;
        if let Some(metadata) = metadata {
            if let Some(supplementary_id) = &params.supplementary_id {
                let supplementary_id = Uuid::parse_str(supplementary_id)
                    .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
                let supplementary = ctx
                    .content
                    .metadata_supplementary
                    .get_supplementary(&supplementary_id)
                    .await
                    .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
                (metadata, supplementary)
            } else {
                (metadata, None)
            }
        } else {
            return Err((StatusCode::FORBIDDEN, "Forbidden".to_owned()))?;
        }
    } else if params.supplementary_id.is_some() {
        let (metadata, supplementary) = ctx
            .check_metadata_supplementary_action_principal(
                &principal,
                &principal_groups,
                &id,
                PermissionAction::View,
            )
            .await
            .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?;
        (metadata, Some(supplementary))
    } else {
        (
            ctx.check_metadata_content_action_principal(
                &principal,
                &principal_groups,
                &id,
                PermissionAction::View,
            )
            .await
            .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?,
            None,
        )
    };
    if metadata.deleted
        && !ctx
            .has_admin_account()
            .await
            .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?
    {
        return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
    }
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
    let buf = ctx.storage.get_buffer(&path).await;
    let buf = match buf {
        Ok(buf) => buf,
        Err(e) => {
            return match e {
                object_store::Error::NotFound { path: _, source: _ } => Err((StatusCode::NOT_FOUND, "Not Found".to_owned())),
                _ => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        }
    };
    let body = Body::from_stream(buf);
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
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

#[tracing::instrument(skip(ctx, headers, params, multipart))]
pub async fn metadata_upload(
    State(ctx): State<BoscaContext>,
    headers: HeaderMap,
    Query(params): Query<Params>,
    mut multipart: Multipart,
) -> Result<(StatusCode, HeaderMap, String), (StatusCode, String)> {
    let principal = get_principal_from_headers(&ctx, &headers)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let principal_groups = ctx.security.get_principal_groups(&principal.id).await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let id = Uuid::parse_str(params.id.as_ref().unwrap().as_str())
        .map_err(|_| (StatusCode::BAD_REQUEST, "Bad Request".to_owned()))?;
    let metadata = ctx
        .check_metadata_action_principal(&principal, &principal_groups, &id, PermissionAction::Edit)
        .await
        .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?;
    let supplementary = get_supplementary(&ctx, &params)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error".to_owned()))?;
    if let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        let path = ctx
            .storage
            .get_metadata_path(&metadata, supplementary.as_ref().map(|s| s.id))
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
        let len = upload_field(&ctx, path, &mut field).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Server Error: {e:?}").to_owned()))?;
        if let Some(supplementary) = &supplementary {
            let content_type = field.content_type().unwrap_or("");
            ctx.content
                .metadata_supplementary
                .set_supplementary_uploaded(&ctx, &supplementary.id, content_type, len)
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Server Error".to_owned()))?;
        } else {
            let content_type = field.content_type().map(|s| s.to_owned());
            let file_name = field.file_name().map(|s| s.to_owned());
            ctx.content
                .metadata
                .set_uploaded(&ctx, &id, &file_name, &content_type, len)
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Server Error".to_owned()))?;
            if params.ready.is_some() && params.ready.unwrap() {
                let mut request = EnqueueRequest {
                    workflow_id: Some(METADATA_PROCESS.to_string()),
                    metadata_id: Some(id),
                    metadata_version: Some(metadata.version),
                    ..Default::default()
                };
                ctx.workflow
                    .enqueue_workflow(&ctx, &mut request)
                    .await
                    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Server Error".to_owned()))?;
            }
        }
    }
    if params.redirect.is_some() {
        let redirect = params.redirect.unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Location",
            HeaderValue::from_str(redirect.as_str()).unwrap(),
        );
        Ok((
            StatusCode::SEE_OTHER,
            headers,
            "Upload successful".to_owned(),
        ))
    } else {
        Ok((
            StatusCode::CREATED,
            HeaderMap::new(),
            "Upload successful".to_owned(),
        ))
    }
}
