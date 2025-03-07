use std::sync::atomic::AtomicI32;

pub mod storage;
pub mod delete;
pub mod transition;
pub mod signed_url;
pub mod profile;
pub mod security;

pub static RUNNING_BACKGROUND: AtomicI32 = AtomicI32::new(0);