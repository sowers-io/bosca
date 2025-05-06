use crate::context::BoscaContext;
use crate::util::profile::add_oauth_principal;
use axum::body::Body;
use axum::extract::{Query, State};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use http::{HeaderMap, StatusCode};
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
    scope: String,
    code: String,
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
    let jar = jar
        .add(
            Cookie::build(("_batov", oauth2_request.verifier.into_secret()))
                .max_age(time::Duration::seconds(60))
                .domain(ctx.security_oauth2.domain.clone())
                .build(),
        )
        .add(
            Cookie::build(("_batos", oauth2_request.token.into_secret()))
                .max_age(time::Duration::seconds(60))
                .domain(ctx.security_oauth2.domain.clone())
                .build(),
        )
        .add(
            Cookie::build(("_bator", to))
                .max_age(time::Duration::seconds(60))
                .domain(ctx.security_oauth2.domain.clone())
                .build(),
        )
        .add(
            Cookie::build(("_batot", params.oauth2_type))
                .max_age(time::Duration::seconds(60))
                .domain(ctx.security_oauth2.domain.clone())
                .build(),
        );
    Ok((
        StatusCode::TEMPORARY_REDIRECT,
        hdrs,
        jar,
        Body::from("Redirecting..."),
    ))
}

#[tracing::instrument(skip(ctx, params))]
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
    let response = ctx
        .security_oauth2
        .exchange_authorization_code(&oauth2_type, verifier, &params.code)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.message))?;
    let access_token = response.access_token().clone().into_secret();
    let account = ctx
        .security_oauth2
        .get_account(&oauth2_type, &access_token)
        .await
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Failed to get Oauth2 Account".to_string(),
            )
        })?;
    let jar = if let Some(id) = account.id() {
        let principal = if let Ok(principal) = ctx
            .security
            .get_principal_by_identifier_oauth2(&id, oauth2_type)
            .await
        {
            principal
        } else {
            let (principal, _) = add_oauth_principal(&ctx, &account, &response, true)
                .await
                .map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        "Failed to create Principal".to_string(),
                    )
                })?;
            principal
        };
        let token = ctx.security.new_token(&principal).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Failed to create Principal Token".to_string(),
            )
        })?;
        CookieJar::new().add(
            Cookie::build(("_bat", token.token))
                .http_only(false)
                .path("/")
                .max_age(time::Duration::seconds(
                    (token.expires_at - token.issued_at) as i64,
                ))
                .build(),
        )
    } else {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Failed to get Oauth2 Account".to_string(),
        ));
    };
    let mut hdrs = HeaderMap::new();
    hdrs.insert("Location", to.to_string().parse().unwrap());
    Ok((StatusCode::FOUND, hdrs, jar, Body::from("Redirecting...")))
}
