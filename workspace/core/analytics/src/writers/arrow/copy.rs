use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use arrow::json::ReaderBuilder;
use parquet::arrow::ArrowWriter;
use crate::writers::arrow::schema::SchemaDefinition;

pub fn copy_to_parquet(json_file: File, schema: Arc<SchemaDefinition>, writer: Arc<Mutex<ArrowWriter<File>>>) -> Result<bool, Box<dyn std::error::Error>> {
    let buf = BufReader::new(json_file);
    let mut writer = writer.lock().unwrap();
    let mut reader = ReaderBuilder::new(schema.schema.clone()).build(buf).unwrap();
    let mut has_records = false;
    loop {
        match reader.next() {
            Some(batch) => {
                let batch = batch?;
                writer.write(&batch)?;
                has_records = true;
            }
            None => {
                return Ok(has_records);
            }
        }
    }
}