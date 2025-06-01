use crate::context::BoscaContext;
use crate::models::security::permission::PermissionAction;
use crate::util::security::get_principal_from_headers;
use axum::body::{to_bytes, Body};
use axum::extract::{Query, Request, State};
use http::{HeaderMap, StatusCode};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Params {
    id: String,
    version: i32,
}

pub async fn get_document_collaboration(
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
    let metadata_id = Uuid::parse_str(params.id.as_str())
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid id".to_owned()))?;
    ctx.check_metadata_version_principal_action(
        &principal,
        &principal_groups,
        &metadata_id,
        params.version,
        PermissionAction::Edit,
    ).await.map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let collaboration = ctx.content.documents.get_document_collaboration(&metadata_id, params.version).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error saving document".to_owned()))?;
    if let Some(collaboration) = collaboration {
        let mut hdrs = HeaderMap::new();
        hdrs.insert("Cache-Control", "private".parse().unwrap());
        Ok((hdrs, Body::from(collaboration.content)))
    } else {
        Err((StatusCode::NOT_FOUND, "Not Found".to_owned()))
    }
}

pub async fn set_document_collaboration(
    State(ctx): State<BoscaContext>,
    Query(params): Query<Params>,
    headers: HeaderMap,
    request: Request<Body>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let principal = get_principal_from_headers(&ctx, &headers)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let principal_groups = ctx
        .security
        .get_principal_groups(&principal.id)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let metadata_id = Uuid::parse_str(params.id.as_str())
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid id".to_owned()))?;
    ctx.check_metadata_version_principal_action(
        &principal,
        &principal_groups,
        &metadata_id,
        params.version,
        PermissionAction::Edit,
    ).await.map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()))?;
    let body = request.into_body();
    let bytes = to_bytes(body, usize::MAX).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error ready body".to_owned()))?;
    let collaboration = bytes.to_vec();
    ctx.content.documents.set_document_collaboration(&metadata_id, params.version, &collaboration).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error saving document".to_owned()))?;
    Ok((StatusCode::OK, "OK".to_owned()))
}