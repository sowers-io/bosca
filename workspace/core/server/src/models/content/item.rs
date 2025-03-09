use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait ContentItem {
    fn id(&self) -> &Uuid;
    fn version(&self) -> Option<i32>;
    fn workflow_state_id(&self) -> &str;
    fn workflow_state_pending_id(&self) -> &Option<String>;
    fn etag(&self) -> &Option<String>;
    fn modified(&self) -> &DateTime<Utc>;
    fn ready(&self) -> &Option<DateTime<Utc>>;
}