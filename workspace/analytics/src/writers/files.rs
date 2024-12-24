use crate::writers::arrow::copy::copy_to_parquet;
use crate::writers::arrow::parquet::writer::new_arrow_writer;
use crate::writers::arrow::schema::SchemaDefinition;
use crate::writers::writer::EventsWriter;
use bytes::{Buf, BytesMut};
use chrono::Utc;
use log::{error, info};
use object_store::aws::AmazonS3Builder;
use object_store::path::Path;
use object_store::{MultipartUpload, ObjectStore};
use std::error::Error;
use std::fs;
use std::fs::{create_dir_all, File};
#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt;
#[cfg(target_os = "macos")]
use std::os::macos::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
use std::path::PathBuf;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicI64};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::task;
use ulid::Ulid;

#[derive(Clone)]
pub struct Config {
    pub temp_dir: String,
    pub batches_dir: String,
    pub pending_objects_dir: String,
    pub max_file_size: u64,
    // TODO: store this on disk so that it survives restarts
    pub last_full_sync: Arc<AtomicI64>,
}

const MAX_UPLOAD_CHUNK_SIZE: usize = 5242880;

pub fn find_file(index: usize, config: Config) -> Result<String, Box<dyn Error>> {
    if !PathBuf::from(&config.temp_dir).exists() {
        create_dir_all(&config.temp_dir)?;
    }
    if !PathBuf::from(&config.batches_dir).exists() {
        create_dir_all(&config.batches_dir)?;
    }
    let paths = fs::read_dir(&config.temp_dir)?;
    let prefix = format!("{}/events-{index}-", config.temp_dir);
    for path in paths.flatten() {
        if path.file_type()?.is_file() {
            let name = path.file_name().into_string().unwrap();
            if name.starts_with(&prefix) && name.ends_with(".json") {
                #[cfg(target_os = "linux")]
                let size = path.metadata()?.st_size();
                #[cfg(target_os = "macos")]
                let size = path.metadata()?.st_size();
                #[cfg(windows)]
                let size = path.metadata()?.file_size();
                if size < config.max_file_size {
                    return Ok(path.file_name().into_string().unwrap());
                }
            }
        }
    }
    Ok(format!(
        "{}/events-{index}-{}.json",
        &config.temp_dir,
        Utc::now().timestamp_millis()
    ))
}

pub async fn watch_files(
    writer: Arc<EventsWriter>,
    schema: Arc<SchemaDefinition>,
    config: Config,
    watching: Arc<AtomicBool>,
) {
    loop {
        if writer.is_stopped() {
            // TODO: is it necessary to interrupt the sleep if this happens?
            break;
        }
        watching.store(true, Relaxed);
        if let Ok(exists) = tokio::fs::try_exists(&config.temp_dir).await {
            if !exists {
                tokio::fs::create_dir_all(&config.temp_dir).await.unwrap();
            }
        }
        if let Ok(exists) = tokio::fs::try_exists(&config.batches_dir).await {
            if !exists {
                tokio::fs::create_dir_all(&config.batches_dir)
                    .await
                    .unwrap();
            }
        }
        if let Ok(exists) = tokio::fs::try_exists(&config.pending_objects_dir).await {
            if !exists {
                tokio::fs::create_dir_all(&config.pending_objects_dir)
                    .await
                    .unwrap();
            }
        }
        if let Err(err) = watch_json(&writer, &schema, &config, false).await {
            error!("error watching json: {:?}", err);
        }
        if let Err(err) = watch_objects(&config).await {
            error!("error watching objects: {:?}", err);
        }
        watching.store(false, Relaxed);
        tokio::time::sleep(Duration::from_secs(15)).await;
    }
}

pub async fn watch_files_hourly(
    writer: Arc<EventsWriter>,
    schema: Arc<SchemaDefinition>,
    config: Config,
    watching: Arc<AtomicBool>,
) {
    loop {
        if writer.is_stopped() {
            // TODO: is it necessary to interrupt the sleep if this happens?
            break;
        }
        // TODO: once stored on disk, move this last
        tokio::time::sleep(Duration::from_secs(3600)).await;
        let now = Utc::now().timestamp_millis();
        if now > config.last_full_sync.load(Relaxed) + 3600000 {
            error!("running hourly watch");
            // TODO: Store on disk
            watching.store(true, Relaxed);
            if let Err(err) = watch_json(&writer, &schema, &config, true).await {
                error!("hourly error watching json: {:?}", err);
            }
            if let Err(err) = watch_objects(&config).await {
                error!("hourly error watching objects: {:?}", err);
            }
            watching.store(false, Relaxed);
        }
    }
}

async fn watch_objects(config: &Config) -> Result<(), Box<dyn Error>> {
    if let Ok(mut read) = tokio::fs::read_dir(&config.pending_objects_dir).await {
        // TODO: make this more generic for multiple object stores
        let s3 = AmazonS3Builder::from_env().build()?;
        while let Ok(Some(entry)) = read.next_entry().await {
            if let Ok(file_type) = entry.file_type().await {
                if file_type.is_file() {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if file_name.ends_with(".parquet") {
                            info!(
                                "processing upload for: {}/{}",
                                config.pending_objects_dir, file_name
                            );
                            let metadata = entry.metadata().await?;
                            let created = metadata.created()?;
                            let utc = time::OffsetDateTime::UNIX_EPOCH
                                + time::Duration::try_from(
                                    created.duration_since(std::time::UNIX_EPOCH).unwrap(),
                                )
                                .unwrap();
                            let path = Path::parse(format!(
                                "ingest/raw/{}/{}/{}/events-{}.parquet",
                                utc.year(),
                                utc.month() as u8,
                                utc.day(),
                                Ulid::new().to_string(),
                            ))?;
                            let mut upload = s3.put_multipart(&path).await?;
                            let mut buf = BytesMut::with_capacity(MAX_UPLOAD_CHUNK_SIZE);
                            let file_name = format!("{}/{}", config.pending_objects_dir, file_name);
                            let mut file = tokio::fs::File::open(&file_name).await?;
                            let len = file.metadata().await?.len();
                            let mut offset = 0;
                            while offset < len {
                                let chunk_len = file.read_buf(&mut buf).await?;
                                let buf_len = buf.len();
                                if buf_len >= MAX_UPLOAD_CHUNK_SIZE {
                                    let copy = buf.copy_to_bytes(buf_len);
                                    buf.clear();
                                    upload.put_part(copy.into()).await?;
                                }
                                offset += chunk_len as u64;
                            }
                            if !buf.is_empty() {
                                let copy = buf.copy_to_bytes(buf.len());
                                buf.clear();
                                upload.put_part(copy.into()).await?;
                            }
                            upload.complete().await?;
                            if let Err(err) = tokio::fs::remove_file(&file_name).await {
                                return Err(
                                    format!("error deleting file: {} {:?}", file_name, err).into()
                                );
                            }
                        }
                    }
                }
            }
        }
    } else {
        return Err("error processing object files".to_string().into());
    }
    Ok(())
}

async fn watch_json(
    writer: &Arc<EventsWriter>,
    schema: &Arc<SchemaDefinition>,
    config: &Config,
    ignore_file_size: bool,
) -> Result<(), Box<dyn Error>> {
    if let Ok(mut read) = tokio::fs::read_dir(&config.temp_dir).await {
        let mut files = Vec::new();
        let mut file_sizes = 0u64;
        while let Ok(Some(entry)) = read.next_entry().await {
            if let Ok(file_type) = entry.file_type().await {
                if file_type.is_file() {
                    if let Ok(metadata) = entry.metadata().await {
                        #[cfg(unix)]
                        let size = metadata.st_size();
                        #[cfg(windows)]
                        let size = metadata.file_size();
                        file_sizes += size;
                    }
                    files.push(entry);
                }
            }
        }
        if (ignore_file_size || file_sizes >= config.max_file_size) && file_sizes > 0 {
            writer.recycle().await;
            config
                .last_full_sync
                .store(Utc::now().timestamp_millis(), Relaxed);
            let parquet_file = format!(
                "{}/batch-{}.parquet",
                &config.batches_dir,
                Utc::now().timestamp_millis()
            );
            let finished_parquet_file = format!(
                "{}/batch-{}.parquet",
                &config.pending_objects_dir,
                Utc::now().timestamp_millis()
            );
            let writer = Arc::new(Mutex::new(
                new_arrow_writer(Arc::clone(schema), &parquet_file, 10000)?,
            ));
            let mut success = true;
            for file in &files {
                if let Ok(file_name) = file.file_name().into_string() {
                    if file_name.starts_with("events-") && file_name.ends_with(".json") {
                        info!("adding json file to parquet: {}", file_name);
                        let spawn_file = format!("{}/{}", &config.temp_dir, file_name);
                        let spawn_writer = Arc::clone(&writer);
                        let spawn_writer_schema = Arc::clone(schema);
                        success = task::spawn_blocking(move || {
                            match File::open(spawn_file) {
                                Ok(file) => {
                                    match copy_to_parquet(file, spawn_writer_schema, spawn_writer) {
                                        Ok(has_records) => {
                                            return has_records
                                        }
                                        Err(err) => {
                                            error!("error copying file to parquet: {:?}", err);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("error opening file for parquet copy: {:?}", e);
                                    return false;
                                }
                            }
                            true
                        })
                        .await
                        .unwrap_or_else(|e| {
                            error!("error copying file: {:?}", e);
                            false
                        }) || success;
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
                            if let Err(err) = tokio::fs::remove_file(format!(
                                "{}/{}",
                                &config.temp_dir, file_name
                            ))
                            .await
                            {
                                return Err(format!("error deleting file: {:?}", err).into());
                            }
                        }
                    }
                }
                if let Err(err) = tokio::fs::rename(parquet_file, finished_parquet_file).await {
                    return Err(format!("error deleting file: {:?}", err).into());
                }
            } else if let Err(err) = tokio::fs::remove_file(parquet_file).await {
                return Err(format!("error deleting file: {:?}", err).into());
            }
        }
    } else {
        return Err("error processing files".to_string().into());
    }
    Ok(())
}
