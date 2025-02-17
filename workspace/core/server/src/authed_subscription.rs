use crate::schema::BoscaSchema;
use crate::security::authorization_extension::{get_anonymous_principal, get_auth_header, get_cookie_header, get_principal};
use async_graphql::http::ALL_WEBSOCKET_PROTOCOLS;
use async_graphql::Data;
use async_graphql_axum::{GraphQLProtocol, GraphQLWebSocket};
use axum::body::{Body, HttpBody};
use axum::extract::{FromRequestParts, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures_util::future::BoxFuture;
use http::{Request, Response};
use std::convert::Infallible;
use std::task::{Context, Poll};
use tower::Service;
use crate::context::BoscaContext;

pub struct AuthGraphQLSubscription {
    executor: BoscaSchema,
    context: BoscaContext,
}

impl Clone for AuthGraphQLSubscription {
    fn clone(&self) -> Self {
        Self {
            executor: self.executor.clone(),
            context: self.context.clone(),
        }
    }
}

impl AuthGraphQLSubscription {
    pub fn new(executor: BoscaSchema, context: BoscaContext) -> Self {
        Self { executor, context }
    }
}

impl<B> Service<Request<B>> for AuthGraphQLSubscription
where
    B: HttpBody + Send + 'static,
{
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = BoxFuture<'static, async_graphql::Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut Context<'_>,
    ) -> Poll<async_graphql::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let executor = self.executor.clone();
        let mut new_ctx = self.context.clone();

        Box::pin(async move {
            let mut data = Data::default();

            if let Some(auth) = get_auth_header(req.headers()) {
                new_ctx.principal = get_principal(&auth, &new_ctx.security)
                    .await
                    .unwrap_or(get_anonymous_principal());
                data.insert(new_ctx);
            } else if let Some(auth) = get_cookie_header(req.headers()) {
                new_ctx.principal = get_principal(&auth, &new_ctx.security)
                    .await
                    .unwrap_or(get_anonymous_principal());
                data.insert(new_ctx);
            }

            let (mut parts, _body) = req.into_parts();

            let protocol = match GraphQLProtocol::from_request_parts(&mut parts, &()).await {
                Ok(protocol) => protocol,
                Err(err) => return Ok(err.into_response()),
            };
            let upgrade = match WebSocketUpgrade::from_request_parts(&mut parts, &()).await {
                Ok(protocol) => protocol,
                Err(err) => return Ok(err.into_response()),
            };

            let resp = upgrade
                .protocols(ALL_WEBSOCKET_PROTOCOLS)
                .on_upgrade(move |stream| {
                    GraphQLWebSocket::new(stream, executor, protocol).with_data(data).serve()
                });
            Ok(resp.into_response())
        })
    }
}

