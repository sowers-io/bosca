use async_graphql::*;
use bytes::{BufMut, BytesMut};
use chrono::prelude::*;
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde_json::Value;
use std::error::Error;
use std::fmt::Debug;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::item::ContentItem;

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq)]
pub enum MetadataType {
    Standard,
    Variant,
}

#[derive(Clone)]
pub struct Metadata {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub version: i32,
    pub metadata_type: MetadataType,
    pub name: String,
    pub content_type: String,
    pub content_length: Option<i64>,
    pub language_tag: String,
    pub labels: Vec<String>,
    pub attributes: Value,
    pub system_attributes: Option<Value>,
    pub item_attributes: Option<Value>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub workflow_state_id: String,
    pub workflow_state_pending_id: Option<String>,
    pub source_id: Option<Uuid>,
    pub source_identifier: Option<String>,
    pub delete_workflow_id: Option<String>,
    pub uploaded: Option<DateTime<Utc>>,
    pub ready: Option<DateTime<Utc>>,
    pub public: bool,
    pub public_content: bool,
    pub public_supplementary: bool
}

impl ContentItem for Metadata {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn version(&self) -> Option<i32> {
        Some(self.version)
    }

    fn workflow_state_id(&self) -> &str {
        &self.workflow_state_id
    }

    fn workflow_state_pending_id(&self) -> &Option<String> {
        &self.workflow_state_pending_id
    }

    fn ready(&self) -> &Option<DateTime<Utc>> {
        &self.ready
    }
}

#[derive(InputObject)]
pub struct MetadataWorkflowInput {
    pub state: String,
    pub delete_workflow_id: Option<String>,
}

#[derive(InputObject)]
pub struct MetadataSourceInput {
    pub id: String,
    pub identifier: String,
}

#[derive(InputObject)]
pub struct MetadataInput {
    pub parent_collection_id: Option<String>,
    pub parent_id: Option<String>,
    pub version: Option<i32>,
    pub metadata_type: Option<MetadataType>,
    pub name: String,
    pub content_type: String,
    pub content_length: Option<i64>,
    pub language_tag: String,
    pub labels: Option<Vec<String>>,
    pub trait_ids: Option<Vec<String>>,
    pub category_ids: Option<Vec<String>>,
    pub attributes: Option<Value>,
    pub state: Option<MetadataWorkflowInput>,
    pub source: Option<MetadataSourceInput>,
    pub index: Option<bool>,
}

impl From<&Row> for Metadata {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            parent_id: row.get("parent_id"),
            version: row.get("version"),
            metadata_type: row.get("type"),
            name: row.get("name"),
            content_type: row.get("content_type"),
            content_length: row.get("content_length"),
            language_tag: row.get("language_tag"),
            labels: row.get("labels"),
            attributes: row.get("attributes"),
            system_attributes: row.get("system_attributes"),
            item_attributes: row.try_get("item_attributes").unwrap_or(None),
            created: row.get("created"),
            modified: row.get("modified"),
            workflow_state_id: row.get("workflow_state_id"),
            workflow_state_pending_id: row.get("workflow_state_pending_id"),
            source_id: row.get("source_id"),
            source_identifier: row.get("source_identifier"),
            delete_workflow_id: row.get("delete_workflow_id"),
            uploaded: row.get("uploaded"),
            ready: row.get("ready"),
            public: row.get("public"),
            public_content: row.get("public_content"),
            public_supplementary: row.get("public_supplementary"),
        }
    }
}

impl<'a> FromSql<'a> for MetadataType {
    fn from_sql(_: &Type, raw: &'a [u8]) -> Result<MetadataType, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        if e == "variant" {
            return Ok(MetadataType::Variant);
        }
        Ok(MetadataType::Standard)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "metadata_type"
    }
}

impl ToSql for MetadataType {
    fn to_sql(&self, _: &Type, w: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        if *self == MetadataType::Variant {
            w.put_slice("variant".as_ref());
        } else {
            w.put_slice("standard".as_ref());
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "metadata_type"
    }

    to_sql_checked!();
}
