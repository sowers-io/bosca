use crate::context::BoscaContext;
use async_graphql::Error;
use axum::extract::State;
use axum::{
    extract::Form,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use log::{error, info};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

#[derive(Deserialize)]
pub struct SignedRequest {
    signed_request: String,
}

#[derive(Serialize)]
pub struct DeletionResponse {
    url: String,
    confirmation_code: String,
    error: Option<String>,
}

#[derive(Deserialize)]
pub struct SignedRequestData {
    user_id: String,
}

pub async fn oauth2_facebook_deauthorize_status() -> impl IntoResponse {
    (StatusCode::OK, "Success")
}

async fn delete_attributes(ctx: &BoscaContext, user_id: &str) -> Result<(), Error> {
    let principal = ctx
        .security
        .get_principal_by_identifier_oauth2(user_id, "facebook")
        .await?;
    if let Some(profile) = ctx.profile.get_by_principal(&principal.id).await? {
        let attributes = ctx.profile.get_attributes(&profile.id).await?;
        for attr in attributes {
            if attr.source == "facebook" {
                ctx.profile
                    .delete_profile_attribute(&profile.id, &attr.id)
                    .await?;
            }
        }
    }
    Ok(())
}

pub async fn oauth2_facebook_deauthorize(
    State(ctx): State<BoscaContext>,
    Form(payload): Form<SignedRequest>,
) -> impl IntoResponse {
    match parse_signed_request(&ctx, &payload.signed_request) {
        Ok(data) => {
            info!("Starting deletion for user_id: {}", data.user_id);
            if let Err(e) = delete_attributes(&ctx, &data.user_id).await {
                let response = DeletionResponse {
                    url: "".to_string(),
                    confirmation_code: "".to_string(),
                    error: Some(format!("Error deleting attributes: {e:?}").to_string()),
                };
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
            }
            let confirmation_code = hex::encode(Uuid::new_v4().as_bytes());
            let status_url = format!(
                "https://{}/oauth2/facebook/deauthorize/status?id={}",
                ctx.security_oauth2.domain, confirmation_code
            )
            .to_string();
            let response = DeletionResponse {
                url: status_url,
                confirmation_code,
                error: None,
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            error!("Error parsing signed request: {e:?}");
            let response = DeletionResponse {
                url: "".to_string(),
                confirmation_code: "".to_string(),
                error: Some(format!("Error parsing signed request: {e:?}").to_string()),
            };
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}

fn parse_signed_request(
    ctx: &BoscaContext,
    signed_request: &str,
) -> Result<SignedRequestData, Error> {
    let parts: Vec<&str> = signed_request.split('.').collect();
    if parts.len() != 2 {
        return Err("Invalid signed request format".into());
    }
    let encoded_sig = parts[0];
    let payload = parts[1];
    let Some(secret) = ctx.security_oauth2.get_facebook_client_secret() else {
        return Err("invalid client secret".into());
    };
    let sig = base64_url_decode(encoded_sig)?;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())?;
    mac.update(payload.as_bytes());
    let expected_sig = mac.finalize().into_bytes();
    if sig != expected_sig.as_slice() {
        return Err("Bad Signed JSON signature!".into());
    }
    let payload_json = base64_url_decode(payload)?;
    let payload_str =
        String::from_utf8(payload_json).map_err(|_| "Invalid UTF-8 in payload".to_string())?;
    let data: SignedRequestData =
        serde_json::from_str(&payload_str).map_err(|e| format!("JSON parsing error: {e}"))?;
    Ok(data)
}

fn base64_url_decode(input: &str) -> Result<Vec<u8>, String> {
    let standard_base64 = input.replace('-', "+").replace('_', "/");
    let padded = match standard_base64.len() % 4 {
        0 => standard_base64,
        2 => format!("{standard_base64}=="),
        3 => format!("{standard_base64}="),
        _ => return Err("Invalid base64 length".to_string()),
    };
    general_purpose::STANDARD
        .decode(padded)
        .map_err(|e| format!("Base64 decoding error: {e}"))
}
