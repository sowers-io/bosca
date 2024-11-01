use std::fs::File;
use std::sync::Arc;
use parquet::arrow::ArrowWriter;
use parquet::basic::{Compression, ZstdLevel};
use parquet::file::properties::WriterProperties;
use crate::events::Events;
use crate::writers::parquet::batch_accumulator::BatchAccumulator;
use crate::writers::parquet::schema::create_schema;

pub struct ParquetWriter {
    writer: ArrowWriter<File>,
    accumulator: BatchAccumulator,
}

impl ParquetWriter {
    pub fn new(path: &str, batch_size: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let schema = Arc::new(create_schema());  // Same schema as before
        let props = WriterProperties::builder()
            .set_compression(Compression::ZSTD(ZstdLevel::default()))
            .set_max_row_group_size(batch_size)
            .build();

        let file = File::create(path)?;
        let writer = ArrowWriter::try_new(file, Arc::clone(&schema), Some(props))?;
        let accumulator = BatchAccumulator::new(Arc::clone(&schema), batch_size);

        Ok(Self {
            writer,
            accumulator,
        })
    }

    pub fn flush(&mut self, force: bool) -> Result<(), Box<dyn std::error::Error>> {
        if self.accumulator.is_full() || (force && !self.accumulator.is_empty()) {
            let batch = self.accumulator.build()?;
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
        self.writer.close()?;
        Ok(())
    }
}
