use std::sync::atomic::AtomicI32;

pub mod yaml;
pub mod storage;
pub mod delete;
pub mod transition;
pub mod signed_url;

pub static RUNNING_BACKGROUND: AtomicI32 = AtomicI32::new(0);