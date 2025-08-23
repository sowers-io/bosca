use crate::graphql::analytics::fake_data::FakeDataGenerator;
use crate::graphql::analytics::types::*;
use async_graphql::Object;

pub struct ContentAnalyticsObject {}

impl ContentAnalyticsObject {
    pub fn new() -> Self {
        Self {}
    }
}

#[Object(name = "ContentAnalytics")]
impl ContentAnalyticsObject {
    async fn content_views(
        &self,
        _ctx: &async_graphql::Context<'_>,
        content_id: Option<String>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<Vec<ContentViewRecord>> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_content_views(content_id, date_range))
    }

    async fn top_content(
        &self,
        _ctx: &async_graphql::Context<'_>,
        limit: Option<i32>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<Vec<TopContentRecord>> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_top_content(limit, date_range))
    }

    async fn content_engagement(
        &self,
        _ctx: &async_graphql::Context<'_>,
        content_id: Option<String>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<Vec<EngagementRecord>> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_content_engagement(content_id, date_range))
    }

    async fn content_performance(
        &self,
        _ctx: &async_graphql::Context<'_>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<ContentMetrics> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_content_metrics(date_range))
    }
}