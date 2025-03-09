use crate::schema::BoscaSchema;
use crate::security::authorization_extension::{get_auth_header, get_cookie_header};
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::response;
use axum::response::IntoResponse;
use http::HeaderMap;

pub async fn graphiql_handler() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

pub async fn graphql_handler(
    State(schema): State<BoscaSchema>,
    headers: HeaderMap,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = request.into_inner();
    if let Some(data) = get_auth_header(&headers) {
        request = request.data(data);
    } else if let Some(data) = get_cookie_header(&headers) {
        request = request.data(data);
    }
    schema.execute(request).await.into()
}
