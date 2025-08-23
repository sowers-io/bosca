use crate::graphql::analytics::fake_data::FakeDataGenerator;
use crate::graphql::analytics::types::*;
use async_graphql::Object;

pub struct SessionAnalyticsObject {}

impl SessionAnalyticsObject {
    pub fn new() -> Self {
        Self {}
    }
}

#[Object(name = "SessionAnalytics")]
impl SessionAnalyticsObject {
    async fn session_totals(
        &self,
        _ctx: &async_graphql::Context<'_>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<Vec<SessionTotalRecord>> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_session_totals(date_range))
    }

    async fn average_session_duration(
        &self,
        _ctx: &async_graphql::Context<'_>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<Vec<SessionDurationRecord>> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_session_durations(date_range))
    }

    async fn sessions_by_device(
        &self,
        _ctx: &async_graphql::Context<'_>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<Vec<SessionDeviceRecord>> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_sessions_by_device(date_range))
    }

    async fn session_metrics(
        &self,
        _ctx: &async_graphql::Context<'_>,
        date_range: Option<DateRange>,
    ) -> async_graphql::Result<SessionMetrics> {
        let mut generator = FakeDataGenerator::new();
        Ok(generator.generate_session_metrics(date_range))
    }
}