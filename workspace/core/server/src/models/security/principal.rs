use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Debug, Formatter};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Principal {
    pub id: Uuid,
    pub verified: bool,
    pub anonymous: bool,
    pub attributes: Value,
    pub verification_token: Option<String>,
}

impl Debug for Principal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("Principal")
            .field(&self.id)
            .field(&self.verified)
            .field(&self.anonymous)
            .finish()
    }
}

impl Principal {
    pub fn new(
        id: Uuid,
        verified: bool,
        anonymous: bool,
        attributes: Value,
    ) -> Self {
        Self {
            id,
            verified,
            anonymous,
            attributes,
            verification_token: None,
        }
    }
}

impl From<&Row> for Principal {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            verified: row.get("verified"),
            anonymous: row.get("anonymous"),
            attributes: row.get("attributes"),
            verification_token: row.get("verification_token"),
        }
    }
}
