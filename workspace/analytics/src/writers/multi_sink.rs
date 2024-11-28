use crate::events::Events;
use crate::events_sink::{EventPipelineContext, EventSink};
use std::error::Error;
use crate::events_transform::EventTransform;

pub struct MultiSink {
    transforms: Vec<Box<dyn EventTransform + Send + Sync + 'static>>,
    sinks: Vec<Box<dyn EventSink + Send + Sync + 'static>>,
}

impl MultiSink {
    #[allow(dead_code)]
    pub fn new(
        transforms: Vec<Box<dyn EventTransform + Send + Sync + 'static>>,
        sinks: Vec<Box<dyn EventSink + Send + Sync + 'static>>,
    ) -> Self {
        Self { transforms, sinks }
    }
}

#[async_trait::async_trait]
impl EventSink for MultiSink {
    async fn add(&mut self, context: &mut EventPipelineContext, events: &Events) -> Result<(), Box<dyn Error>> {
        if self.transforms.is_empty() {
            for sink in self.sinks.iter_mut() {
                sink.add(context, events).await?;
            }
        } else {
            let mut events = events.clone();
            for transform in self.transforms.iter() {
                transform.transform(context, &mut events).await?;
            }
            for sink in self.sinks.iter_mut() {
                sink.add(context, &events).await?;
            }
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
