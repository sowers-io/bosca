use crate::context::BoscaContext;
use crate::models::security::permission::PermissionAction;
use crate::util::security::get_principal_from_headers;
use axum::body::Body;
use axum::extract::{Multipart, Query};
use axum::extract::{Request, State};
use http::{HeaderMap, HeaderValue, StatusCode};
use log::error;
use serde::Deserialize;
use uuid::Uuid;
use crate::util::upload::upload_field;

#[derive(Debug, Deserialize)]
pub struct Params {
    supplementary_id: String,
    redirect: Option<String>,
}

#[tracing::instrument(skip(ctx, params, headers, request))]
pub async fn collection_download(
    State(ctx): State<BoscaContext>,
    Query(params): Query<Params>,
    headers: HeaderMap,
    request: Request<Body>,
) -> Result<(HeaderMap, Body), (StatusCode, String)> {
    let principal = get_principal_from_headers(&ctx, &headers)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let url = format!(
        "/files/collection{}",
        request.uri().path_and_query().unwrap()
    );
    let supplementary_id = Uuid::parse_str(&params.supplementary_id).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_owned(),
        )
    })?;
    let (collection, supplementary) = if ctx.security.verify_signed_url(&url) {
        let supplementary = if let Some(supplementary) = ctx
            .content
            .collection_supplementary
            .get_supplementary(&supplementary_id)
            .await
            .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?
        {
            supplementary
        } else {
            return Err((StatusCode::FORBIDDEN, "Forbidden".to_owned()))?;
        };
        let Some(collection) = ctx
            .content
            .collections
            .get(&supplementary.collection_id)
            .await
            .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?
        else {
            return Err((StatusCode::FORBIDDEN, "Forbidden".to_owned()));
        };
        (collection, supplementary)
    } else {
        ctx.check_collection_supplementary_action_principal(
            &principal,
            &supplementary_id,
            PermissionAction::View,
        )
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
    let path = ctx
        .storage
        .get_collection_path(&collection, Some(supplementary.id))
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

#[tracing::instrument(skip(ctx, headers, params, multipart))]
pub async fn collection_upload(
    State(ctx): State<BoscaContext>,
    headers: HeaderMap,
    Query(params): Query<Params>,
    mut multipart: Multipart,
) -> Result<(StatusCode, HeaderMap, String), (StatusCode, String)> {
    let principal = get_principal_from_headers(&ctx, &headers)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let supplementary_id = Uuid::parse_str(&params.supplementary_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Bad Request".to_owned()))?;
    let (collection, supplementary) = ctx
        .check_collection_supplementary_action_principal(
            &principal,
            &supplementary_id,
            PermissionAction::Edit,
        )
        .await
        .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden".to_owned()))?;
    if let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        let path = ctx
            .storage
            .get_collection_path(&collection, Some(supplementary.id))
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
        let len = upload_field(&ctx, path, &mut field).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Server Error: {:?}", e).to_owned()))?;
        let content_type = field.content_type().unwrap_or("");
        ctx.content
            .metadata_supplementary
            .set_supplementary_uploaded(&ctx, &supplementary.id, content_type, len)
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
