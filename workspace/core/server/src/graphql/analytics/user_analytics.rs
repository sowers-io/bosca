use crate::graphql::analytics::fake_data::FakeDataGenerator;
use crate::graphql::analytics::types::*;
use async_graphql::Object;

pub struct UserAnalyticsObject {}

impl UserAnalyticsObject {
    pub fn new() -> Self {
        Self {}
    }
}

#[Object(name = "UserAnalytics")]
impl UserAnalyticsObject {
    async fn daily_active_users(
        &self,
        _ctx: &async_graphql::Context<'_>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<Vec<DailyActiveUserRecord>> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_daily_active_users(date_range))
    }

    async fn weekly_active_users(
        &self,
        _ctx: &async_graphql::Context<'_>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<Vec<WeeklyActiveUserRecord>> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_weekly_active_users(date_range))
    }

    async fn new_users(
        &self,
        _ctx: &async_graphql::Context<'_>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<Vec<NewUserRecord>> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_new_users(date_range))
    }

    async fn returning_users(
        &self,
        _ctx: &async_graphql::Context<'_>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<Vec<ReturningUserRecord>> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_returning_users(date_range))
    }

    async fn retention_flow(
        &self,
        _ctx: &async_graphql::Context<'_>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<RetentionFlowResponse> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_retention_flow(date_range))
    }
}