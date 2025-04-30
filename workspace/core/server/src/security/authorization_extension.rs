use crate::models::security::principal::Principal;
use crate::security::authorization_data::{AuthorizationData, AuthorizationDataType};
use async_graphql::extensions::{
    Extension, ExtensionContext, ExtensionFactory, NextPrepareRequest,
};
use async_graphql::{Error, Request, ServerResult};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use http::HeaderMap;
use serde_json::Value;
use std::any::TypeId;
use std::sync::Arc;
use axum_extra::extract::CookieJar;
use uuid::Uuid;
use crate::context::BoscaContext;
use crate::datastores::security::SecurityDataStore;

pub struct Authorization;

impl ExtensionFactory for Authorization {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(AuthorizationExtension::default())
    }
}

#[derive(Default, Debug)]
struct AuthorizationExtension {}

pub fn get_anonymous_principal() -> Principal {
    Principal::new(Uuid::nil(), false, true, Value::Null, vec![])
}

pub fn get_auth_header(headers: &HeaderMap) -> Option<AuthorizationData> {
    headers.get("Authorization").and_then(|value| {
        value
            .to_str()
            .map(|s| AuthorizationData::new(AuthorizationDataType::Header, s.to_string()))
            .ok()
    })
}

pub fn get_cookie_header(headers: &HeaderMap) -> Option<AuthorizationData> {
    let jar = CookieJar::from_headers(headers);
    jar.get("_bat").map(|value| AuthorizationData::new(AuthorizationDataType::Cookie, value.value().to_string()))
}

pub async fn get_principal(
    authorization: &AuthorizationData,
    datastore: &SecurityDataStore,
) -> Result<Principal, Error> {
    Ok(match authorization.data_type {
        AuthorizationDataType::Header => {
            let authorization = authorization.data.as_str();
            if authorization.starts_with("Bearer ") {
                let token = &authorization["Bearer ".len()..authorization.len()];
                datastore
                    .get_principal_by_token(token)
                    .await
                    .unwrap_or(get_anonymous_principal())
            } else if authorization.starts_with("Basic ") {
                let token = &authorization["Basic ".len()..authorization.len()];
                let value: String = String::from_utf8(BASE64_STANDARD.decode(token)?)?;
                let mut parts = value.split(":");
                let username = parts.next().unwrap();
                let password = parts.next().unwrap();
                datastore
                    .get_principal_by_password(username, password)
                    .await
                    .unwrap_or(get_anonymous_principal())
            } else {
                get_anonymous_principal()
            }
        }
        AuthorizationDataType::Cookie => {
            datastore
                .get_principal_by_cookie(authorization.data.as_str())
                .await
                .unwrap_or(get_anonymous_principal())
        }
    })
}

#[async_trait::async_trait]
impl Extension for AuthorizationExtension {
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        request: Request,
        next: NextPrepareRequest<'_>,
    ) -> ServerResult<Request> {
        match request.data.get(&TypeId::of::<AuthorizationData>()) {
            Some(data) => {
                let data: Option<&AuthorizationData> = data.downcast_ref();
                match data {
                    Some(data) => {
                        let bctx = ctx.data::<BoscaContext>().unwrap();
                        let mut new_ctx = bctx.clone();
                        let principal = get_principal(data, &bctx.security).await;
                        if let Ok(principal) = principal {
                            new_ctx.principal = principal;    
                        } else {
                            new_ctx.principal = get_anonymous_principal(); 
                        }
                        if !new_ctx.principal.anonymous {
                            new_ctx.principal_groups = bctx.security.get_principal_groups(&new_ctx.principal.id).await.unwrap_or(vec![]);
                        }
                        next.run(ctx, request.data(new_ctx)).await
                    }
                    None => next.run(ctx, request).await,
                }
            }
            None => next.run(ctx, request).await,
        }
    }
}
