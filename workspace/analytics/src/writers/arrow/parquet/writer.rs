use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use log::info;
use parquet::arrow::ArrowWriter;
use parquet::basic::{Compression, ZstdLevel};
use parquet::file::properties::WriterProperties;
use crate::events::Events;
use crate::writers::arrow::batch_accumulator::BatchAccumulator;
use crate::writers::arrow::schema::SchemaDefinition;

pub struct ParquetWriter {
    writer: ArrowWriter<File>,
    accumulator: BatchAccumulator,
}

pub fn new_arrow_writer(schema: Arc<SchemaDefinition>, path: &str, batch_size: usize) -> Result<ArrowWriter<File>, Box<dyn std::error::Error>> {
    let props = WriterProperties::builder()
        .set_compression(Compression::ZSTD(ZstdLevel::default()))
        .set_max_row_group_size(batch_size)
        .build();
    let file = File::create(path)?;
    Ok(ArrowWriter::try_new(file, Arc::clone(&schema.schema), Some(props))?)
}

impl ParquetWriter {
    #[allow(dead_code)]
    pub fn new(schema: Arc<SchemaDefinition>, path: &str, batch_size: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let writer = new_arrow_writer(Arc::clone(&schema), path, batch_size)?;
        let accumulator = BatchAccumulator::new(schema, batch_size);
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
            self.writer.flush()?;
            self.writer.inner_mut().flush()?;
            info!("flushed events: {} -> {}", batch.num_rows(), self.writer.bytes_written());
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
