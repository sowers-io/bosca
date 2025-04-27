use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub ip: String,
    pub port: u16,
}

impl Node {
    pub fn new(id: Uuid, ip: String, port: u16) -> Self {
        Self { id, ip, port }
    }
}
