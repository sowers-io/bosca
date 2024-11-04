use std::error::Error;
use crate::events::Events;
use crate::events_sink::EventSink;

pub struct MultiSink {
    sinks: Vec<Box<dyn EventSink + Send + Sync + 'static>>,
}

impl MultiSink {
    #[allow(dead_code)]
    pub fn new(sinks: Vec<Box<dyn EventSink + Send + Sync + 'static>>) -> Self {
        Self { sinks }
    }
}

#[async_trait::async_trait]
impl EventSink for MultiSink {
    async fn add(&mut self, events: Events) -> Result<(), Box<dyn Error>> {
        for sink in self.sinks.iter_mut() {
            sink.add(events.clone()).await?;
        }
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), Box<dyn Error>> {
        for sink in self.sinks.iter_mut() {
            sink.flush().await?;
        }
        Ok(())
    }

    async fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        for sink in self.sinks.iter_mut() {
            sink.finish().await?;
        }
        Ok(())
    }
}