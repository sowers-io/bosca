use crate::context::BoscaContext;
use crate::models::security::credentials::{Credential, CredentialType};
use crate::models::security::credentials_oauth2::Oauth2Credential;
use crate::security::token::Token;
use async_graphql::Error;
use axum::body::Body;
use axum::extract::{Query, State};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use http::{HeaderMap, StatusCode};
use log::error;
use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RedirectParams {
    to: Option<String>,
    #[serde(rename = "type")]
    oauth2_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallbackParams {
    state: String,
    code: Option<String>,
    scope: Option<String>,
    authuser: Option<String>,
    hd: Option<String>,
    prompt: Option<String>,
}

#[tracing::instrument(skip(ctx, params, jar))]
pub async fn oauth2_redirect(
    State(ctx): State<BoscaContext>,
    Query(params): Query<RedirectParams>,
    jar: CookieJar,
) -> Result<(StatusCode, HeaderMap, CookieJar, Body), (StatusCode, String)> {
    let to = params.to.unwrap_or_else(|| "/".to_string());
    if !ctx.security_oauth2.is_internal_redirect_url(&to) {
        return Err((StatusCode::BAD_REQUEST, "Invalid Redirect".to_string()));
    }
    let oauth2_request = ctx
        .security_oauth2
        .new_default_redirect_url(&params.oauth2_type)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.message))?;
    let mut hdrs = HeaderMap::new();
    hdrs.insert("Location", oauth2_request.url.to_string().parse().unwrap());
    hdrs.insert("Cache-Control", "private".parse().unwrap());
    let jar = jar
        .add(
            Cookie::build(("_batov", oauth2_request.verifier.into_secret()))
                .max_age(time::Duration::seconds(60))
                .domain(ctx.security_oauth2.domain.clone())
                .path("/")
                .build(),
        )
        .add(
            Cookie::build(("_batos", oauth2_request.token.into_secret()))
                .max_age(time::Duration::seconds(60))
                .domain(ctx.security_oauth2.domain.clone())
                .path("/")
                .build(),
        )
        .add(
            Cookie::build(("_bator", to))
                .max_age(time::Duration::seconds(60))
                .domain(ctx.security_oauth2.domain.clone())
                .path("/")
                .build(),
        )
        .add(
            Cookie::build(("_batot", params.oauth2_type))
                .max_age(time::Duration::seconds(60))
                .domain(ctx.security_oauth2.domain.clone())
                .path("/")
                .build(),
        );
    Ok((
        StatusCode::TEMPORARY_REDIRECT,
        hdrs,
        jar,
        Body::from("Redirecting..."),
    ))
}

pub async fn oauth2_callback_internal(
    ctx: &BoscaContext,
    params: &CallbackParams,
    verifier: &str,
    oauth2_type: &str,
) -> Result<Token, Error> {
    let Some(code) = &params.code else {
      // TODO: redirect somewhere?
      return Err(Error::new("Missing code"));
    };
    let response = ctx
        .security_oauth2
        .exchange_authorization_code(oauth2_type, verifier, code)
        .await?;
    let access_token = response.access_token().clone().into_secret();
    let account = ctx
        .security_oauth2
        .get_account(oauth2_type, &access_token)
        .await?;
    if let Some(id) = account.id() {
        let principal = if let Ok(principal) = ctx
            .security
            .get_principal_by_identifier_oauth2(id, oauth2_type)
            .await
        {
            let credentials = ctx
                .security
                .get_principal_credentials(&principal.id)
                .await?;
            let mut credentials = credentials
                .into_iter()
                .filter(|c| {
                    c.get_type() == CredentialType::Oauth2
                        && c.identifier_type().unwrap_or_default() == oauth2_type
                })
                .collect::<Vec<_>>();
            let credential = credentials.first_mut().unwrap();
            credential.set_tokens(response)?;
            ctx.security
                .set_principal_credential(&principal.id, credential)
                .await?;
            let attributes = serde_json::to_value(&account)?;
            ctx.security
                .merge_principal_attributes(&principal.id, attributes)
                .await?;
            principal
        } else {
            let profile = account.new_profile()?;
            let Some(profile) = profile else {
                return Err(Error::new("Failed to create Profile"));
            };
            let credential = Oauth2Credential::new(&account, Some(&response))?;
            let credential = Credential::Oauth2(credential);
            let (principal, profile_id) = ctx
                .security
                .add_principal_with_credential(
                    ctx,
                    &credential,
                    &profile,
                    Some(account.verified()),
                    true,
                    true,
                )
                .await?;
            let principal = ctx.security.get_principal_by_id(&principal).await?;
            if let Err(e) = ctx.profile.update_storage(ctx, &profile_id).await {
                error!("Failed to update profile storage: {e:?}");
            }
            principal
        };
        Ok(ctx.security.new_token(&principal)?)
    } else {
        Err(Error::new(
            "Failed to get Oauth2 Account, missing account id",
        ))
    }
}

#[tracing::instrument(skip(ctx, params, jar))]
pub async fn oauth2_callback(
    State(ctx): State<BoscaContext>,
    Query(params): Query<CallbackParams>,
    jar: CookieJar,
) -> Result<(StatusCode, HeaderMap, CookieJar, Body), (StatusCode, String)> {
    let to = jar.get("_bator").map(|c| c.value()).unwrap_or_else(|| "/");
    let state = if let Some(cookie) = jar.get("_batos") {
        cookie.value()
    } else {
        ""
    };
    if state != params.state {
        return Err((StatusCode::BAD_REQUEST, "Invalid state".to_string()));
    }
    let verifier = if let Some(cookie) = jar.get("_batov") {
        cookie.value()
    } else {
        ""
    };
    let oauth2_type = if let Some(cookie) = jar.get("_batot") {
        cookie.value()
    } else {
        ""
    };
    let token = match oauth2_callback_internal(&ctx, &params, verifier, oauth2_type).await {
        Ok(token) => token,
        Err(e) => {
            error!("Failed to exchange authorization code: {e:?}");
            let mut hdrs = HeaderMap::new();
            if to.contains('?') {
                hdrs.insert(
                    "Location",
                    format!("{}&error={}", to, e.message).parse().unwrap(),
                );
            } else {
                hdrs.insert(
                    "Location",
                    format!("{}?error={}", to, e.message).parse().unwrap(),
                );
            }
            hdrs.insert("Cache-Control", "private".parse().unwrap());
            return Ok((StatusCode::FOUND, hdrs, jar, Body::from("Redirecting...")));
        }
    };
    let new_jar = CookieJar::new().add(
        Cookie::build(("_bat", token.token))
            .http_only(false)
            .domain(ctx.security_oauth2.domain.clone())
            .path("/")
            .max_age(time::Duration::seconds(
                (token.expires_at - token.issued_at) as i64,
            ))
            .build(),
    );
    let mut hdrs = HeaderMap::new();
    hdrs.insert("Location", to.to_string().parse().unwrap());
    hdrs.insert("Cache-Control", "private".parse().unwrap());
    Ok((
        StatusCode::FOUND,
        hdrs,
        new_jar,
        Body::from("Redirecting..."),
    ))
}
