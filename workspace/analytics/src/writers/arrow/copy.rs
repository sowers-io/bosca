use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use arrow::json::ReaderBuilder;
use crate::writers::arrow::parquet::writer::new_arrow_writer;
use crate::writers::arrow::schema::SchemaDefinition;

#[allow(dead_code)]
fn copy_to_parquet(json_file: File, parquet_file: &str, schema: Arc<SchemaDefinition>, batch_size: usize) -> Result<(), Box<dyn std::error::Error>> {
    let buf = BufReader::new(json_file);
    let mut reader = ReaderBuilder::new(schema.schema.clone()).build(buf).unwrap();
    let mut writer = new_arrow_writer(schema, parquet_file, batch_size)?;
    loop {
        match reader.next() {
            Some(batch) => {
                let batch = batch?;
                writer.write(&batch)?;
            }
            None => {
                writer.finish()?;
                return Ok(());
            }
        }
    }
}