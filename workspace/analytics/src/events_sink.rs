use http::HeaderMap;
use crate::events::Events;

#[derive(Clone)]
pub struct EventPipelineContext {
    headers: HeaderMap,
}

impl EventPipelineContext {
    pub fn new(headers: HeaderMap) -> Self {
        Self { headers }
    }

    pub fn get_header_value(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|v| v.to_str().unwrap())
    }
}

#[async_trait::async_trait]
pub trait EventSink {

    async fn add(&mut self, context: &mut EventPipelineContext, events: &Events) -> Result<(), Box<dyn std::error::Error>>;
    async fn flush(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn finish(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}