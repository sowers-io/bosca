use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI32};
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use log::{error, info};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc::error::SendError;
use tokio::time::timeout;
use crate::events::Events;
use crate::events_sink::{EventSink, EventPipelineContext};

pub struct WriterPayload {
    context: EventPipelineContext,
    events: Events,
}

pub struct WriterWorker {
    stopped: Arc<AtomicBool>,
    active: Arc<AtomicI32>,
    sink: Option<Box<dyn EventSink + Send + Sync + 'static>>,
    sender: Option<Sender<WriterPayload>>,
    queue_size: usize,
}

impl WriterWorker {
    pub fn new(stopped: Arc<AtomicBool>, active: Arc<AtomicI32>, sink: Box<dyn EventSink + Send + Sync + 'static>, queue_size: usize) -> Self {
        Self {
            stopped,
            active,
            sink: Some(sink),
            sender: None,
            queue_size,
        }
    }

    pub async fn write(&self, context: EventPipelineContext, events: Events) -> Result<(), SendError<WriterPayload>> {
        let payload = WriterPayload {
            context,
            events,
        };
        self.sender.as_ref().unwrap().send(payload).await
    }

    pub fn start(&mut self) {
        let (send, recv) = mpsc::channel(self.queue_size);
        self.sender = Some(send);

        let active = Arc::clone(&self.active);
        let stopped = Arc::clone(&self.stopped);
        let sink = std::mem::take(&mut self.sink).unwrap();

        active.fetch_add(1, Relaxed);
        tokio::spawn(Self::process(stopped, active, recv, sink));
    }

    async fn process(stopped: Arc<AtomicBool>, active: Arc<AtomicI32>, mut recv: Receiver<WriterPayload>, mut sink: Box<dyn EventSink + Send + Sync>) {
        let mut done = false;
        while !done && !stopped.load(Relaxed) && !recv.is_closed() {
            match timeout(Duration::from_millis(3000), recv.recv()).await {
                Ok(Some(mut payload)) => {
                    if let Err(error) = sink.add(&mut payload.context, &payload.events).await {
                        error!("error adding events to sink: {error:?}");
                    }
                }
                Ok(None) => {
                    info!("shutting down worker");
                    done = true;
                }
                Err(_) => {
                    if let Err(error) = sink.flush().await {
                        error!("error finishing adding events to sink: {error:?}");
                    }
                    if recv.is_closed() {
                        done = true;
                    }
                }
            }
        }
        if let Err(e) = sink.finish().await {
            error!("error finishing sink: {e:?}");
        }
        active.fetch_add(-1, Relaxed);
    }
}