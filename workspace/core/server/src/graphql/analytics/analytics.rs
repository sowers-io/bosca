use crate::graphql::analytics::resolvers::AnalyticQueriesResolvers;
use async_graphql::Object;

pub struct AnalyticQueriesObject {}

impl AnalyticQueriesObject {
    pub fn new() -> Self {
        Self {}
    }
}

#[Object(name = "AnalyticQueries")]
impl AnalyticQueriesObject {
    async fn events(
        &self,
        ctx: &async_graphql::Context<'_>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> async_graphql::Result<Vec<crate::graphql::analytics::types::AnalyticEvent>> {
        let resolver = AnalyticQueriesResolvers;
        resolver.events(ctx, offset, limit).await
    }

    async fn execute(
        &self,
        ctx: &async_graphql::Context<'_>,
        request: crate::graphql::analytics::types::AnalyticQueryRequest,
    ) -> async_graphql::Result<crate::graphql::analytics::types::AnalyticQueryResponse> {
        let resolver = AnalyticQueriesResolvers;
        resolver.execute(ctx, request).await
    }
}