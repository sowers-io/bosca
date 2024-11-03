use crate::events::Events;

#[async_trait::async_trait]
pub trait EventSink {

    async fn add(&mut self, events: Events) -> Result<(), Box<dyn std::error::Error>>;
    async fn flush(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn finish(self) -> Result<(), Box<dyn std::error::Error>>;
}