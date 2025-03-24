use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::content::collection::Collection;
use crate::models::content::metadata::Metadata;

pub trait ContentItem {
    fn id(&self) -> &Uuid;
    fn version(&self) -> Option<i32>;
    fn workflow_state_id(&self) -> &str;
    fn workflow_state_pending_id(&self) -> &Option<String>;
    fn etag(&self) -> &Option<String>;
    fn modified(&self) -> &DateTime<Utc>;
    fn ready(&self) -> &Option<DateTime<Utc>>;

    fn as_collection(&self) -> Option<&Collection>;
    fn as_metadata(&self) -> Option<&Metadata>;
}