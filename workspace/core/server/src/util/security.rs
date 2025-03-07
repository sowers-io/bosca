use crate::context::BoscaContext;
use crate::models::security::principal::Principal;
use crate::security::authorization_extension::{
    get_anonymous_principal, get_auth_header, get_cookie_header, get_principal,
};
use async_graphql::Error;
use http::HeaderMap;

pub async fn get_principal_from_headers(
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
