use crate::graphql::analytics::resolvers::AnalyticQueriesResolvers;
use crate::graphql::analytics::user_analytics::UserAnalyticsObject;
use crate::graphql::analytics::session_analytics::SessionAnalyticsObject;
use crate::graphql::analytics::content_analytics::ContentAnalyticsObject;
use crate::graphql::analytics::device_analytics::DeviceAnalyticsObject;
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

    async fn users(&self) -> UserAnalyticsObject {
        UserAnalyticsObject::new()
    }

    async fn sessions(&self) -> SessionAnalyticsObject {
        SessionAnalyticsObject::new()
    }

    async fn content(&self) -> ContentAnalyticsObject {
        ContentAnalyticsObject::new()
    }

    async fn devices(&self) -> DeviceAnalyticsObject {
        DeviceAnalyticsObject::new()
    }
}