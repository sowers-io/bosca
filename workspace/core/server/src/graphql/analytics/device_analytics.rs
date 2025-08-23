use crate::graphql::analytics::fake_data::FakeDataGenerator;
use crate::graphql::analytics::types::*;
use async_graphql::Object;

pub struct DeviceAnalyticsObject {}

impl DeviceAnalyticsObject {
    pub fn new() -> Self {
        Self {}
    }
}

#[Object(name = "DeviceAnalytics")]
impl DeviceAnalyticsObject {
    async fn device_types(
        &self,
        _ctx: &async_graphql::Context<'_>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<AnalyticQueryResponse> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_device_types_enhanced(date_range))
    }

    async fn device_adoption_trends(
        &self,
        _ctx: &async_graphql::Context<'_>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<AnalyticQueryResponse> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_device_adoption_trends(date_range))
    }

    async fn device_performance_metrics(
        &self,
        _ctx: &async_graphql::Context<'_>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<AnalyticQueryResponse> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_device_performance_metrics(date_range))
    }

    async fn device_user_behavior(
        &self,
        _ctx: &async_graphql::Context<'_>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<AnalyticQueryResponse> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_device_user_behavior(date_range))
    }
}