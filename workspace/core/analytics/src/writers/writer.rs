use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use tokio::sync::mpsc::error::SendError;
use bosca_pool::Pool;
use crate::events::Events;
use crate::events_sink::{EventSink, EventPipelineContext};
use crate::writers::worker::{WriterPayload, WriterWorker};

pub struct EventsWriter {
    active: Arc<AtomicI32>,
    stopped: Arc<AtomicBool>,
    pool: Pool<WriterWorker>,
}

impl EventsWriter {
    pub async fn new(pool_size: usize, worker_queue_size: usize, sink_factory: impl Fn(usize) -> Result<Box<dyn EventSink + Send + Sync>, Box<dyn Error>> + Send + Sync + 'static) -> Self {
        let active = Arc::new(AtomicI32::new(0));
        let stopped = Arc::new(AtomicBool::new(false));
        let worker_active = Arc::clone(&active);
        let worker_stopped = Arc::clone(&stopped);
        let pool = Pool::new(pool_size, move |index| {
            let sink = sink_factory(index).unwrap();
            let mut worker = WriterWorker::new(Arc::clone(&worker_stopped), Arc::clone(&worker_active), sink, worker_queue_size);
            worker.start();
            worker
        }).await;
        Self {
            active,
            stopped,
            pool,
        }
    }

    pub async fn recycle(&self) {
        self.pool.recycle().await
    }

    pub async fn stop(&self) {
        self.stopped.store(true, Relaxed);
        self.pool.close().await;
    }
    
    pub fn is_stopped(&self) -> bool {
        self.stopped.load(Relaxed)
    }

    pub fn is_active(&self) -> bool {
        self.active.load(Relaxed) > 0
    }

    pub async fn write(&self, context: EventPipelineContext, events: Events) -> Result<(), SendError<WriterPayload>> {
        assert!(!self.stopped.load(Relaxed));
        let worker = self.pool.acquire().await;
        match worker.object.write(context, events).await {
            Ok(_) => {
                assert!(self.pool.release(worker).await.err().is_none());
                Ok(())
            },
            Err(e) => {
                assert!(self.pool.release(worker).await.err().is_none());
                Err(e)
            }
        }
    }
}