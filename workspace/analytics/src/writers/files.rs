use std::error::Error;
use std::fs;
use std::fs::{create_dir_all, exists, File};
use std::os::linux::fs::MetadataExt;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use chrono::Utc;
use log::{error, info};
use tokio::task;
use crate::writers::arrow::copy::copy_to_parquet;
use crate::writers::arrow::parquet::writer::new_arrow_writer;
use crate::writers::arrow::schema::SchemaDefinition;
use crate::writers::writer::EventsWriter;

//const MAX_FILE_SIZE: u64 = 104857600;
const MAX_FILE_SIZE: u64 = 100;
const TEMP_DIR: &str = "./analytics/temp";
const BATCHES_DIR: &str = "./analytics/batches";

pub fn find_file(index: usize) -> Result<String, Box<dyn Error>> {
    if !exists(TEMP_DIR)? {
        create_dir_all(TEMP_DIR)?;
    }
    if !exists(BATCHES_DIR)? {
        create_dir_all(BATCHES_DIR)?;
    }
    let paths = fs::read_dir(TEMP_DIR)?;
    let prefix = format!("{}/events-{index}-", TEMP_DIR);
    for path in paths {
        if let Ok(path) = path {
            if path.file_type()?.is_file() {
                let name = path.file_name().into_string().unwrap();
                if name.starts_with(&prefix) && name.ends_with(".json") && path.metadata()?.st_size() < MAX_FILE_SIZE {
                    return Ok(path.file_name().into_string().unwrap());
                }
            }
        }
    }
    Ok(format!("{}/events-{index}-{}.json", TEMP_DIR, Utc::now().timestamp_millis()))
}

pub async fn watch_files(writer: Arc<EventsWriter>, schema: Arc<SchemaDefinition>) {
    loop {
        info!("watching files");
        if let Ok(exists) = tokio::fs::try_exists(TEMP_DIR).await {
            if !exists {
                tokio::fs::create_dir_all(TEMP_DIR).await.unwrap();
            }
        }
        if let Ok(exists) = tokio::fs::try_exists(BATCHES_DIR).await {
            if !exists {
                tokio::fs::create_dir_all(BATCHES_DIR).await.unwrap();
            }
        }
        if let Ok(mut read) = tokio::fs::read_dir(TEMP_DIR).await {
            let mut files = Vec::new();
            let mut recycle = false;
            while let Ok(Some(entry)) = read.next_entry().await {
                if let Ok(file_type) = entry.file_type().await {
                    if file_type.is_file() {
                        if let Ok(metadata) = entry.metadata().await {
                            if metadata.st_size() >= MAX_FILE_SIZE {
                                recycle = true;
                            }
                        }
                        files.push(entry);
                    }
                }
            }
            if recycle {
                writer.recycle().await;
                let parquet_file = format!("{}/batch-{}.parquet", BATCHES_DIR, Utc::now().timestamp_millis());
                let writer = Arc::new(Mutex::new(new_arrow_writer(Arc::clone(&schema), &parquet_file, 10000).unwrap()));
                let mut success = true;
                for file in &files {
                    if let Ok(file_name) = file.file_name().into_string() {
                        if file_name.starts_with("events-") && file_name.ends_with(".json") {
                            info!("processing file: {}", file_name);
                            let spawn_file = format!("{}/{}", TEMP_DIR, file_name);
                            let spawn_writer = Arc::clone(&writer);
                            let spawn_writer_schema = Arc::clone(&schema);
                            success = task::spawn_blocking(move || {
                                match File::open(spawn_file) {
                                    Ok(file) => {
                                        if let Err(err) = copy_to_parquet(file, spawn_writer_schema, spawn_writer) {
                                            error!("error copying file to parquet: {:?}", err);
                                        }
                                    },
                                    Err(e) => {
                                        error!("error opening file for parquet copy: {:?}", e);
                                        return false;
                                    }
                                }
                                true
                            }).await.unwrap_or_else(|e| {
                                error!("error copying file: {:?}", e);
                                false
                            });
                            if !success {
                                let _ = writer.lock().unwrap().finish();
                                break;
                            }
                        }
                    }
                }
                if success {
                    let mut writer = writer.lock().unwrap();
                    if let Err(e) = writer.flush() {
                        error!("error flushing parquet records: {:?}", e);
                        success = false;
                    } else if let Err(e) = writer.finish() {
                        error!("error finishing parquet: {:?}", e);
                        success = false;
                    }
                }
                if success {
                    for file in files {
                        if let Ok(file_name) = file.file_name().into_string() {
                            if file_name.starts_with("events-") && file_name.ends_with(".json") {
                                if let Err(err) = tokio::fs::remove_file(format!("{}/{}", TEMP_DIR, file_name)).await {
                                    error!("error deleting file: {:?}", err);
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    if let Err(err) = tokio::fs::remove_file(parquet_file).await {
                        error!("error deleting file: {:?}", err);
                        break;
                    }
                }
            }
        } else {
            error!("error watching files");
        }
        tokio::time::sleep(Duration::from_secs(15)).await;
    }
}