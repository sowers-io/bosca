use crate::context::BoscaContext;
use crate::models::content::supplementary::MetadataSupplementary;
use crate::models::security::permission::PermissionAction;
use crate::models::security::principal::Principal;
use crate::security::authorization_extension::{
    get_anonymous_principal, get_auth_header, get_cookie_header, get_principal,
};
use async_graphql::Error;
use axum::body::Body;
use axum::extract::{Multipart, Query};
use axum::extract::{Request, State};
use bytes::{Buf, BufMut, BytesMut};
use http::{HeaderMap, HeaderValue, StatusCode};
use log::error;
use object_store::MultipartUpload;
use serde::Deserialize;
use std::io::Write;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Params {
    id: Option<String>,
    supplementary_id: Option<String>,
    ready: Option<bool>,
    redirect: Option<String>,
}

async fn get_principal_from_headers(
    ctx: &BoscaContext,
    headers: &HeaderMap,
) -> Result<Principal, Error> {
    if let Some(data) = get_auth_header(headers) {
        Ok(get_principal(&data, &ctx.security).await?)
    } else if let Some(data) = get_cookie_header(headers) {
        Ok(get_principal(&data, &ctx.security).await?)
    } else {
        Ok(get_anonymous_principal())
    }
}

async fn get_supplementary(
    ctx: &BoscaContext,
    params: &Params,
    metadata_id: &Uuid,
) -> Result<Option<MetadataSupplementary>, Error> {
    Ok(if params.supplementary_id.is_none() {
        None
    } else {
        ctx.content
            .metadata
            .get_supplementary(metadata_id, params.supplementary_id.as_ref().unwrap())
            .await?
    })
}

pub async fn download(
    State(ctx): State<BoscaContext>,
    Query(params): Query<Params>,
    headers: HeaderMap,
    request: Request<Body>,
) -> Result<(HeaderMap, Body), (StatusCode, String)> {
    let principal = get_principal_from_headers(&ctx, &headers)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let id_str = params.id.as_deref().unwrap_or("");
    let id = Uuid::parse_str(id_str).map_err(|_| (StatusCode::BAD_REQUEST, "Bad Request".to_owned()))?;
    let url = format!("/files{}", request.uri().path_and_query().unwrap());
    let metadata = if ctx.security.verify_signed_url(&url) {
        let metadata = ctx
            .content
            .metadata
            .get(&id)
            .await
            .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?;
        if metadata.is_none() {
            return Err((StatusCode::FORBIDDEN, "Forbidden".to_owned()))?;
        }
        metadata.unwrap()
    } else {
        ctx.check_metadata_content_action_principal(&principal, &id, PermissionAction::View)
            .await
            .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?
    };
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

pub async fn upload(
    State(ctx): State<BoscaContext>,
    headers: HeaderMap,
    Query(params): Query<Params>,
    mut multipart: Multipart,
) -> Result<(StatusCode, HeaderMap, String), (StatusCode, String)> {
    let principal = get_principal_from_headers(&ctx, &headers)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let id = Uuid::parse_str(params.id.as_ref().unwrap().as_str())
        .map_err(|_| (StatusCode::BAD_REQUEST, "Bad Request".to_owned()))?;
    let metadata = ctx
        .check_metadata_action_principal(&principal, &id, PermissionAction::Edit)
        .await
        .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?;
    let supplementary = get_supplementary(&ctx, &params, &metadata.id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error".to_owned()))?;
    if let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        let path = ctx
            .storage
            .get_metadata_path(&metadata, supplementary.as_ref().map(|s| s.key.clone()))
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
        let mut upload = ctx
            .storage
            .put_multipart(&path)
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
        let mut len = 0;
        let buf = BytesMut::with_capacity(5242880);
        let writer = &mut buf.writer();
        while let Some(chunk) = field
            .chunk()
            .await
            .map_err(|err| {
                error!("Error getting chunk: {}", err);
                (StatusCode::BAD_REQUEST, err.to_string())
            })?
        {
            let chunk_len = chunk.len();
            len += chunk_len;
            let write_len = writer.write(chunk.as_ref()).unwrap();
            if write_len != chunk_len {
                error!("Error validating write {}, {}", write_len, chunk_len);
                return Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid length".to_string()));
            }
            // assert_eq!(write_len, chunk_len);
            let buf_len = writer.get_ref().len();
            if buf_len >= 5242880 {
                let copy = writer.get_mut().copy_to_bytes(buf_len);
                writer.get_mut().clear();
                upload.put_part(copy.into()).await.map_err(|err| {
                    error!("Error putting part {}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                })?;
            }
        }
        let buf_len = writer.get_ref().len();
        if buf_len > 0 {
            let copy = writer.get_mut().copy_to_bytes(buf_len);
            upload
                .put_part(copy.into())
                .await
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        }
        upload
            .complete()
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        if supplementary.is_none() {
            let content_type = field.content_type().map(|s| s.to_owned());
            let file_name = field.file_name().map(|s| s.to_owned());
            ctx.content
                .metadata
                .set_uploaded(&id, &file_name, &content_type, len)
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Server Error".to_owned()))?;
            if params.ready.is_some() && params.ready.unwrap() {
                let process_id = "metadata.process".to_string();
                let workflow = ctx.workflow;
                workflow
                    .enqueue_metadata_workflow(&process_id, &id, &metadata.version, None, None)
                    .await
                    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Server Error".to_owned()))?;
            }
        } else {
            let key = supplementary.unwrap();
            let content_type = field.content_type().unwrap_or("");
            ctx.content
                .metadata
                .set_supplementary_uploaded(&id, &key.key, content_type, len)
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Server Error".to_owned()))?;
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
