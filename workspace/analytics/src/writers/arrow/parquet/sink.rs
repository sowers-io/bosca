use std::sync::Arc;
use crate::events::Events;
use crate::events_sink::EventSink;
use crate::writers::arrow::parquet::writer::ParquetWriter;
use crate::writers::arrow::schema::SchemaDefinition;

pub struct ParquetSink {
    writer: ParquetWriter,
}

impl ParquetSink {
    #[allow(dead_code)]
    pub fn new(schema: Arc<SchemaDefinition>, path: &str, batch_size: usize) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            writer: ParquetWriter::new(schema, path, batch_size)?
        })
    }
}

#[async_trait::async_trait]
impl EventSink for ParquetSink {
    async fn add(&mut self, events: Events) -> Result<(), Box<dyn std::error::Error>> {
        self.writer.write(events)
    }

    async fn flush(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.writer.flush(true)
    }

    async fn finish(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.writer.finish()
    }
}
