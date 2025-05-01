use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CacheConfiguration {
    pub id: String,
    pub max_capacity: u64,
    pub ttl: u64,
    pub tti: u64,
}