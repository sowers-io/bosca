use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use log::warn;
#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};
#[cfg(windows)]
use tokio::signal::windows::ctrl_c;
use crate::util::RUNNING_BACKGROUND;

#[cfg(unix)]
pub async fn shutdown_hook() {
    let mut interrupt = signal(SignalKind::interrupt()).unwrap();
    let mut terminate = signal(SignalKind::terminate()).unwrap();
    tokio::select! {
        _ = interrupt.recv() => {
            warn!("Received SIGINT, shutting down");
            loop {
                if RUNNING_BACKGROUND.load(Relaxed) > 0 {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                } else {
                    break
                }
            }
        },
        _ = terminate.recv() => {
            warn!("Received SIGTERM, shutting down");
            loop {
                if RUNNING_BACKGROUND.load(Relaxed) > 0 {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                } else {
                    break
                }
            }
        }
    }
}

#[cfg(windows)]
pub async fn shutdown_hook() {
    let mut interrupt = ctrl_c().unwrap();
    tokio::select! {
        _ = interrupt.recv() => {
            warn!("Received ctr_c, shutting down");
            loop {
                if RUNNING_BACKGROUND.load(Relaxed) > 0 {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                } else {
                    break
                }
            }
        },
    }
}