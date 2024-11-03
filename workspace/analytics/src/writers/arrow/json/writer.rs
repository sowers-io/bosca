use std::fs::File;
use std::sync::Arc;
use arrow::json::writer::LineDelimited;
use arrow::json::{Writer, WriterBuilder};
use log::info;
use crate::events::Events;
use crate::writers::arrow::batch_accumulator::BatchAccumulator;
use crate::writers::arrow::schema::SchemaDefinition;

pub struct JsonWriter {
    writer: Writer<File, LineDelimited>,
    accumulator: BatchAccumulator,
}

impl JsonWriter {
    pub fn new(schema: Arc<SchemaDefinition>, path: &str, batch_size: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::create(path)?;
        let builder = WriterBuilder::new().with_explicit_nulls(true);
        let writer = builder.build::<_, LineDelimited>(file);
        let accumulator = BatchAccumulator::new(Arc::clone(&schema), batch_size);
        Ok(Self {
            writer,
            accumulator,
        })
    }

    pub fn flush(&mut self, force: bool) -> Result<(), Box<dyn std::error::Error>> {
        if self.accumulator.is_full() || (force && !self.accumulator.is_empty()) {
            let batch = self.accumulator.build()?;
            info!("flushing events: {}", batch.num_rows());
            self.writer.write(&batch)?;
        }
        Ok(())
    }

    pub fn write(&mut self, events: Events) -> Result<(), Box<dyn std::error::Error>> {
        self.accumulator.add_batch(events);
        self.flush(false)?;
        Ok(())
    }

    pub fn finish(mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.flush(true)?;
        self.writer.finish()?;
        Ok(())
    }
}
