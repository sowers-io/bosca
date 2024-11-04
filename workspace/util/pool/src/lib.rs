use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use async_channel::{bounded, Receiver, Sender};
use tokio::time::timeout;

pub struct Pool<T> {
    index: Arc<AtomicI64>,
    sender: Sender<PoolObject<T>>,
    receiver: Receiver<PoolObject<T>>,
    capacity: usize,
    create_fn: Arc<dyn Fn(usize) -> T + Send + Sync + 'static>,
}

pub struct PoolObject<T> {
    index: i64,
    pub object: T,
}

impl<T> Pool<T> {
    pub async fn new(capacity: usize, create_fn: impl Fn(usize) -> T + Send + Sync + 'static) -> Self {
        let (sender, receiver) = bounded(capacity);
        let pool = Pool {
            index: Arc::new(AtomicI64::new(0)),
            sender,
            receiver,
            capacity,
            create_fn: Arc::new(create_fn),
        };
        pool.recycle().await;
        pool
    }

    pub async fn recycle(&self) {
        let index = self.index.fetch_add(1, Relaxed) + 1;
        let create = &self.create_fn;
        while !self.sender.is_empty() {
            let _ = timeout(Duration::from_millis(3000), self.receiver.recv()).await;
        }
        assert!(self.sender.is_empty());
        for i in 0..self.capacity {
            self.sender.send(PoolObject { index, object: create(i) }).await.unwrap();
        }
    }

    pub async fn acquire(&self) -> PoolObject<T> {
        self.receiver.recv().await.unwrap()
    }

    pub async fn release(&self, object: PoolObject<T>) -> Result<(), Box<dyn Error>> {
        if object.index == self.index.load(Relaxed) {
            let _ = timeout(Duration::from_millis(3000), self.sender.send(object)).await?;
        }
        Ok(())
    }
}

impl<T> Clone for Pool<T> {
    fn clone(&self) -> Self {
        Pool {
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
            index: Arc::clone(&self.index),
            capacity: self.capacity,
            create_fn: Arc::clone(&self.create_fn),
        }
    }
}