use std::error::Error;
use crate::events::Events;
use crate::events_sink::EventPipelineContext;

#[async_trait::async_trait]
pub trait EventTransform {
    async fn transform(&self, context: &mut EventPipelineContext, event: &mut Events) -> Result<(), Box<dyn Error>>;
}