use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI64};
use std::sync::atomic::Ordering::Relaxed;
use crate::events::Events;
use crate::writers::parquet::writer::ParquetWriter;

pub async fn process(rx: &mut tokio::sync::mpsc::Receiver<Events>, active: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = ParquetWriter::new("events", 10000)?;
    while let Some(events) = rx.recv().await {
        writer.write(events)?;
    }
    writer.finish()?;
    active.store(false, Relaxed);
    Ok(())
}