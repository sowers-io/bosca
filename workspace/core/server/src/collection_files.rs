use crate::context::BoscaContext;
use crate::models::content::collection_supplementary::CollectionSupplementary;
use crate::models::security::permission::PermissionAction;
use crate::util::security::get_principal_from_headers;
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
    supplementary_id: String,
    redirect: Option<String>,
}

async fn get_supplementary(
    ctx: &BoscaContext,
    params: &Params,
    metadata_id: &Uuid,
) -> Result<Option<CollectionSupplementary>, Error> {
    ctx.content
        .collection_supplementary
        .get_supplementary(metadata_id, &params.supplementary_id)
        .await
}

pub async fn collection_download(
    State(ctx): State<BoscaContext>,
    Query(params): Query<Params>,
    headers: HeaderMap,
    request: Request<Body>,
) -> Result<(HeaderMap, Body), (StatusCode, String)> {
    let principal = get_principal_from_headers(&ctx, &headers)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let id_str = params.id.as_deref().unwrap_or("");
    let id =
        Uuid::parse_str(id_str).map_err(|_| (StatusCode::BAD_REQUEST, "Bad Request".to_owned()))?;
    let url = format!(
        "/files/collection{}",
        request.uri().path_and_query().unwrap()
    );
    let collection = if ctx.security.verify_signed_url(&url) {
        let collection = ctx
            .content
            .collections
            .get(&id)
            .await
            .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?;
        if collection.is_none() {
            return Err((StatusCode::FORBIDDEN, "Forbidden".to_owned()))?;
        }
        collection.unwrap()
    } else {
        ctx.check_collection_supplementary_action_principal(&principal, &id, PermissionAction::View)
            .await
            .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?
    };
    if collection.deleted
        && !ctx
            .has_admin_account()
            .await
            .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?
    {
        return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
    }
    let Some(supplementary) = get_supplementary(&ctx, &params, &collection.id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            )
        })? else {
        return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
    };
    let path = ctx
        .storage
        .get_collection_path(&collection, &supplementary.key)
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
        supplementary.content_type.parse().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            )
        })?,
    );
    Ok((headers, body))
}

pub async fn collection_upload(
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
    let collection = ctx
        .check_collection_action_principal(&principal, &id, PermissionAction::Edit)
        .await
        .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?;
    let Some(supplementary) = get_supplementary(&ctx, &params, &collection.id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error".to_owned()))? else {
        return Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))?;
    };
    if let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        let path = ctx
            .storage
            .get_collection_path(&collection, &supplementary.key)
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
        while let Some(chunk) = field.chunk().await.map_err(|err| {
            error!("Error getting chunk: {}", err);
            (StatusCode::BAD_REQUEST, err.to_string())
        })? {
            let chunk_len = chunk.len();
            len += chunk_len;
            let write_len = writer.write(chunk.as_ref()).unwrap();
            if write_len != chunk_len {
                error!("Error validating write {}, {}", write_len, chunk_len);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Invalid length".to_string(),
                ));
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
        let content_type = field.content_type().unwrap_or("");
        ctx.content
            .metadata_supplementary
            .set_supplementary_uploaded(&ctx, &id, &supplementary.key, supplementary.plan_id, content_type, len)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Server Error".to_owned()))?;
    }
    if let Some(redirect) = params.redirect {
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
