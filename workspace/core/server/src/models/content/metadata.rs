use crate::models::content::document::DocumentInput;
use crate::models::content::document_template::DocumentTemplateInput;
use crate::models::content::item::ContentItem;
use crate::models::content::metadata_profile::MetadataProfileInput;
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
use crate::models::content::collection::Collection;
use crate::models::content::collection_template::CollectionTemplateInput;
use crate::models::content::guide::GuideInput;
use crate::models::content::guide_template::GuideTemplateInput;

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MetadataType {
    Standard,
    Variant,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub version: i32,
    pub active_version: i32,
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
    pub workflow_state_valid: Option<DateTime<Utc>>,
    pub source_id: Option<Uuid>,
    pub source_identifier: Option<String>,
    pub source_url: Option<String>,
    pub delete_workflow_id: Option<String>,
    pub uploaded: Option<DateTime<Utc>>,
    pub ready: Option<DateTime<Utc>>,
    pub public: bool,
    pub public_content: bool,
    pub public_supplementary: bool,
    pub etag: Option<String>,
    pub deleted: bool
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

    fn etag(&self) -> &Option<String> {
        &self.etag
    }

    fn modified(&self) -> &DateTime<Utc> {
        &self.modified
    }

    fn ready(&self) -> &Option<DateTime<Utc>> {
        &self.ready
    }

    fn as_collection(&self) -> Option<&Collection> {
        None
    }

    fn as_metadata(&self) -> Option<&Metadata> {
        Some(&self)
    }
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct MetadataWorkflowInput {
    pub state: String,
    pub delete_workflow_id: Option<String>,
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct MetadataSourceInput {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub source_url: Option<String>
}

#[derive(InputObject, Default, Clone, Serialize, Deserialize)]
pub struct MetadataInput {
    pub slug: Option<String>,
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
    pub document: Option<DocumentInput>,
    pub guide: Option<GuideInput>,
    pub document_template: Option<DocumentTemplateInput>,
    pub collection_template: Option<CollectionTemplateInput>,
    pub guide_template: Option<GuideTemplateInput>,
    pub state: Option<MetadataWorkflowInput>,
    pub source: Option<MetadataSourceInput>,
    pub profiles: Option<Vec<MetadataProfileInput>>,
}

impl From<&Row> for Metadata {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            parent_id: row.get("parent_id"),
            version: row.get("version"),
            active_version: row.get("active_version"),
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
            workflow_state_valid: row.get("workflow_state_valid"),
            source_id: row.get("source_id"),
            source_identifier: row.get("source_identifier"),
            source_url: row.get("source_url"),
            delete_workflow_id: row.get("delete_workflow_id"),
            uploaded: row.get("uploaded"),
            ready: row.get("ready"),
            public: row.get("public"),
            public_content: row.get("public_content"),
            public_supplementary: row.get("public_supplementary"),
            etag: row.get("etag"),
            deleted: row.get("deleted")
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
