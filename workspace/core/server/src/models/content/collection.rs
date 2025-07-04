use async_graphql::*;
use bytes::{BufMut, BytesMut};
use chrono::prelude::*;
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde_json::Value;
use std::error::Error;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::item::ContentItem;
use crate::models::content::metadata::{Metadata, MetadataInput};
use crate::models::content::ordering::{Ordering, OrderingInput};

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum CollectionType {
    Root,
    System,
    Standard,
    Folder,
    Queue,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: Uuid,
    pub collection_type: CollectionType,
    pub name: String,
    pub description: Option<String>,
    pub locked: bool,
    pub items_locked: bool,
    pub labels: Vec<String>,
    pub attributes: Value,
    pub system_attributes: Option<Value>,
    pub item_attributes: Option<Value>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub template_metadata_id: Option<Uuid>,
    pub template_metadata_version: Option<i32>,
    pub workflow_state_id: String,
    pub workflow_state_pending_id: Option<String>,
    pub workflow_state_valid: Option<DateTime<Utc>>,
    pub delete_workflow_id: Option<String>,
    pub public: bool,
    pub public_list: bool,
    pub public_supplementary: bool,
    pub ready: Option<DateTime<Utc>>,
    pub ordering: Option<Vec<Ordering>>,
    pub etag: Option<String>,
    pub deleted: bool,
}

impl ContentItem for Collection {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn version(&self) -> Option<i32> {
        None
    }

    fn workflow_state_id(&self) -> &str {
        &self.workflow_state_id
    }

    fn workflow_state_pending_id(&self) -> &Option<String> {
        &self.workflow_state_pending_id
    }

    fn etag(&self) -> &Option<String> {
        &self.etag
    }

    fn modified(&self) -> &DateTime<Utc> {
        &self.modified
    }

    fn ready(&self) -> &Option<DateTime<Utc>> {
        &self.ready
    }

    fn as_metadata(&self) -> Option<&Metadata> {
        None
    }

    fn as_collection(&self) -> Option<&Collection> {
        Some(self)
    }
}

#[derive(Clone)]
pub struct CollectionChild {
    pub collection_id: Option<Uuid>,
    pub metadata_id: Option<Uuid>,
    pub attributes: Option<Value>,
}

#[derive(InputObject, Default, Clone)]
pub struct CollectionWorkflowInput {
    pub state: String,
    pub delete_workflow_id: Option<String>,
}

#[derive(InputObject, Default)]
pub struct CollectionChildInput {
    pub collection: CollectionInput,
    pub attributes: Option<Value>,
}

#[derive(InputObject, Default)]
pub struct MetadataChildInput {
    pub metadata: MetadataInput,
    pub attributes: Option<Value>,
}

#[derive(InputObject, Default)]
pub struct CollectionInput {
    pub slug: Option<String>,
    pub parent_collection_id: Option<String>,
    pub collection_type: Option<CollectionType>,
    pub name: String,
    pub description: Option<String>,
    pub locked: Option<bool>,
    pub items_locked: Option<bool>,
    pub labels: Option<Vec<String>>,
    pub attributes: Option<Value>,
    pub ordering: Option<Vec<OrderingInput>>,
    pub state: Option<CollectionWorkflowInput>,
    pub trait_ids: Option<Vec<String>>,
    pub category_ids: Option<Vec<String>>,
    pub collections: Option<Vec<CollectionChildInput>>,
    pub metadata: Option<Vec<MetadataChildInput>>,
    pub template_metadata_id: Option<String>,
    pub template_metadata_version: Option<i32>,
}

impl From<&Row> for Collection {
    fn from(row: &Row) -> Self {
        let ordering_value: Option<Value> = row.get("ordering");
        let ordering: Option<Vec<Ordering>> = if let Some(ordering) = ordering_value {
            if ordering.is_null() {
                None
            } else {
                serde_json::from_value(ordering).unwrap()
            }
        } else {
            None
        };
        Self {
            id: row.get("id"),
            collection_type: row.get("type"),
            name: row.get("name"),
            description: row.get("description"),
            locked: row.get("locked"),
            items_locked: row.get("items_locked"),
            labels: row.get("labels"),
            attributes: row.get("attributes"),
            system_attributes: row.get("system_attributes"),
            item_attributes: row.try_get("item_attributes").unwrap_or(None),
            created: row.get("created"),
            modified: row.get("modified"),
            template_metadata_id: row.get("template_metadata_id"),
            template_metadata_version: row.get("template_metadata_version"),
            workflow_state_id: row.get("workflow_state_id"),
            workflow_state_pending_id: row.get("workflow_state_pending_id"),
            workflow_state_valid: row.get("workflow_state_valid"),
            delete_workflow_id: row.get("delete_workflow_id"),
            ready: row.get("ready"),
            public: row.get("public"),
            public_list: row.get("public_list"),
            public_supplementary: row.get("public_supplementary"),
            etag: row.get("etag"),
            deleted: row.get("deleted"),
            ordering,
        }
    }
}

impl From<&Row> for CollectionChild {
    fn from(row: &Row) -> Self {
        Self {
            collection_id: row.get("child_collection_id"),
            metadata_id: row.get("child_metadata_id"),
            attributes: row.get("attributes"),
        }
    }
}

impl<'a> FromSql<'a> for CollectionType {
    fn from_sql(_: &Type, raw: &'a [u8]) -> Result<CollectionType, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        Ok(match e.as_str() {
            "root" => CollectionType::Root,
            "system" => CollectionType::System,
            "standard" => CollectionType::Standard,
            "folder" => CollectionType::Folder,
            "queue" => CollectionType::Queue,
            _ => CollectionType::Standard,
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "collection_type"
    }
}

impl ToSql for CollectionType {
    fn to_sql(&self, _: &Type, w: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            CollectionType::Root => w.put_slice("root".as_ref()),
            CollectionType::System => w.put_slice("system".as_ref()),
            CollectionType::Standard => w.put_slice("standard".as_ref()),
            CollectionType::Folder => w.put_slice("folder".as_ref()),
            CollectionType::Queue => w.put_slice("queue".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "collection_type"
    }

    to_sql_checked!();
}
