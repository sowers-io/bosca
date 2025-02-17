use crate::context::BoscaContext;
use crate::datastores::content::util::build_find_args;
use crate::datastores::notifier::Notifier;
use crate::graphql::content::content::{ExtensionFilterType, FindAttributeInput};
use crate::models::content::collection::MetadataChildInput;
use crate::models::content::metadata::{Metadata, MetadataInput};
use crate::models::content::metadata_relationship::{
    MetadataRelationship, MetadataRelationshipInput,
};
use crate::models::content::search::SearchDocumentInput;
use crate::models::content::supplementary::{MetadataSupplementary, MetadataSupplementaryInput};
use crate::models::security::permission::{Permission, PermissionAction};
use crate::util::storage::index_documents;
use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use log::error;
use serde_json::{Map, Value};
use std::sync::Arc;
use uuid::Uuid;
use crate::models::content::category::Category;
use crate::models::content::metadata_profile::MetadataProfile;

#[derive(Clone)]
pub struct MetadataDataStore {
    pool: Arc<Pool>,
    notifier: Arc<Notifier>,
}

impl MetadataDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

    async fn on_metadata_changed(&self, id: &Uuid) -> Result<(), Error> {
        if let Err(e) = self.notifier.metadata_changed(id).await {
            error!("Failed to notify metadata changes: {:?}", e);
        }
        Ok(())
    }

    async fn on_metadata_supplementary_changed(&self, id: &Uuid, key: &str) -> Result<(), Error> {
        if let Err(e) = self.notifier.metadata_supplementary_changed(id, key).await {
            error!("Failed to notify metadata supplementary changes: {:?}", e);
        }
        Ok(())
    }

    async fn on_collection_changed(&self, id: &Uuid) -> Result<(), Error> {
        if let Err(e) = self.notifier.collection_changed(id).await {
            error!("Failed to notify collection changes: {:?}", e);
        }
        Ok(())
    }

    pub async fn get_slug(&self, id: &Uuid) -> Result<String, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select slug from slugs where metadata_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Err(Error::new("metadata not found"));
        }
        Ok(rows.first().unwrap().get("slug"))
    }

    pub async fn find(
        &self,
        attributes: &[FindAttributeInput],
        content_types: &Option<Vec<String>>,
        category_ids: Option<Vec<Uuid>>,
        extension_filter: Option<ExtensionFilterType>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Metadata>, Error> {
        let connection = self.pool.get().await?;
        let (query, values) = build_find_args(
            "metadata",
            "select m.* from metadata m ",
            "m",
            attributes,
            content_types,
            &category_ids,
            extension_filter,
            &offset,
            &limit,
        );
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get(&self, id: &Uuid) -> Result<Option<Metadata>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from metadata where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    pub async fn get_all(&self, offset: i64, limit: i64) -> Result<Vec<Metadata>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from metadata order by name offset $1 limit $2")
            .await?;
        let rows = connection.query(&stmt, &[&offset, &limit]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_by_version(&self, id: &Uuid, version: i32) -> Result<Option<Metadata>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from metadata_versions where id = $1 and version = $2")
            .await?;
        let rows = connection.query(&stmt, &[id, &version]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    pub async fn get_categories(&self, id: &Uuid) -> Result<Vec<Category>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select c.* from metadata_categories mc inner join categories c on (mc.category_id = c.id) where metadata_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_parent_ids(
        &self,
        id: &Uuid,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Uuid>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select collection_id from collection_items where child_metadata_id = $1 offset $2 limit $3")
            .await?;
        let rows = connection.query(&stmt, &[id, &offset, &limit]).await?;
        Ok(rows.iter().map(|r| r.get("collection_id")).collect())
    }

    pub async fn get_trait_ids(&self, id: &Uuid) -> Result<Vec<String>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select trait_id from metadata_traits where metadata_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows
            .iter()
            .map(|r| {
                let id: String = r.get("trait_id");
                id
            })
            .collect())
    }

    pub async fn get_profiles(&self, id: &Uuid) -> Result<Vec<MetadataProfile>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from metadata_profiles where metadata_id = $1 order by sort asc")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_supplementary(
        &self,
        id: &Uuid,
        key: &String,
    ) -> Result<Option<MetadataSupplementary>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from metadata_supplementary where metadata_id = $1 and key = $2",
            )
            .await?;
        let rows = connection.query(&stmt, &[id, key]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    pub async fn get_supplementaries(
        &self,
        id: &Uuid,
    ) -> Result<Vec<MetadataSupplementary>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from metadata_supplementary where metadata_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn add_supplementary(
        &self,
        supplementary: &MetadataSupplementaryInput,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into metadata_supplementary (metadata_id, key, name, content_type, content_length, attributes, source_id, source_identifier) values ($1, $2, $3, $4, $5, $6, $7, $8)").await?;
        let id = Uuid::parse_str(supplementary.metadata_id.as_str())?;
        let sid = if supplementary.source_identifier.is_some() {
            Some(Uuid::parse_str(
                supplementary.source_identifier.as_ref().unwrap().as_str(),
            )?)
        } else {
            None
        };
        connection
            .execute(
                &stmt,
                &[
                    &id,
                    &supplementary.key,
                    &supplementary.name,
                    &supplementary.content_type,
                    &supplementary.content_length,
                    &supplementary.attributes,
                    &sid,
                    &supplementary.source_identifier,
                ],
            )
            .await?;
        self.on_metadata_supplementary_changed(&id, &supplementary.key).await?;
        Ok(())
    }

    pub async fn set_supplementary_uploaded(
        &self,
        metadata_id: &Uuid,
        key: &str,
        content_type: &str,
        len: usize,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("update metadata_supplementary set uploaded = now(), content_type = $1, content_length = $2 where metadata_id = $3 and key = $4").await?;
        let len: i64 = len as i64;
        let key = key.to_owned();
        let content_type = content_type.to_owned();
        connection
            .execute(&stmt, &[&content_type, &len, &metadata_id, &key])
            .await?;
        self.on_metadata_supplementary_changed(metadata_id, &key).await?;
        Ok(())
    }

    pub async fn delete(&self, metadata_id: &Uuid) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "select collection_id from collection_items where child_metadata_id = $1",
            )
            .await?;
        let rows = txn.query(&stmt, &[metadata_id]).await?;
        let collection_ids: Vec<Uuid> = rows.iter().map(|r| r.get("collection_id")).collect();
        let stmt = txn
            .prepare_cached("delete from metadata where id = $1")
            .await?;
        txn.execute(&stmt, &[&metadata_id]).await?;
        let stmt = txn
            .prepare_cached("delete from metadata_versions where id = $1")
            .await?;
        txn.execute(&stmt, &[&metadata_id]).await?;
        txn.commit().await?;
        self.on_metadata_changed(metadata_id).await?;
        for collection_id in collection_ids {
            self.on_collection_changed(&collection_id).await?;
        }
        Ok(())
    }

    pub async fn delete_supplementary(
        &self,
        metadata_id: &Uuid,
        key: &String,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "delete from metadata_supplementary where metadata_id = $1 and key = $2",
            )
            .await?;
        connection.execute(&stmt, &[&metadata_id, &key]).await?;
        self.on_metadata_supplementary_changed(metadata_id, key).await?;
        Ok(())
    }

    pub async fn set_public(&self, id: &Uuid, public: bool) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata set public = $1, modified = now() where id = $2")
            .await?;
        connection.execute(&stmt, &[&public, id]).await?;
        self.on_metadata_changed(id).await?;
        Ok(())
    }

    pub async fn set_public_content(&self, id: &Uuid, public: bool) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update metadata set public_content = $1, modified = now() where id = $2",
            )
            .await?;
        connection.execute(&stmt, &[&public, id]).await?;
        self.on_metadata_changed(id).await?;
        Ok(())
    }

    pub async fn set_supplementary_public(&self, id: &Uuid, public: bool) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update metadata set public_supplementary = $1, modified = now() where id = $2",
            )
            .await?;
        connection.execute(&stmt, &[&public, id]).await?;
        self.on_metadata_changed(id).await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn add(&self, metadata: &MetadataInput) -> Result<(Uuid, i32, i32), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;

        match self.add_txn(&txn, metadata).await {
            Ok(value) => {
                txn.commit().await?;
                self.on_metadata_changed(&value.0).await?;
                Ok(value)
            }
            Err(err) => {
                txn.rollback().await?;
                Err(err)
            }
        }
    }

    pub async fn edit(&self, id: &Uuid, metadata: &MetadataInput) -> Result<(), Error> {
        let mut source_id: Option<Uuid> = None;
        let mut source_identifier: Option<String> = None;
        if let Some(source) = &metadata.source {
            source_id = Some(Uuid::parse_str(&source.id)?);
            source_identifier = Some(source.identifier.clone());
        }
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;

        let stmt = txn
            .prepare_cached(
                "select collection_id from collection_items where child_metadata_id = $1",
            )
            .await?;
        let rows = txn.query(&stmt, &[id]).await?;
        let collection_ids: Vec<Uuid> = rows.iter().map(|r| r.get("collection_id")).collect();

        match self
            .edit_txn(&txn, id, metadata, &source_id, &source_identifier)
            .await
        {
            Ok(value) => {
                txn.commit().await?;
                self.on_metadata_changed(id).await?;
                for collection_id in collection_ids {
                    self.on_collection_changed(&collection_id).await?;
                }
                Ok(value)
            }
            Err(err) => {
                txn.rollback().await?;
                Err(err)
            }
        }
    }

    pub async fn set_attributes(&self, metadata_id: &Uuid, attributes: Value) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata set attributes = $1, modified = now() where id = $2")
            .await?;
        connection
            .execute(&stmt, &[&attributes, &metadata_id])
            .await?;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    pub async fn set_system_attributes(
        &self,
        metadata_id: &Uuid,
        attributes: Value,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update metadata set system_attributes = $1, modified = now() where id = $2",
            )
            .await?;
        connection
            .execute(&stmt, &[&attributes, &metadata_id])
            .await?;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    pub async fn set_uploaded(
        &self,
        metadata_id: &Uuid,
        original_file_name: &Option<String>,
        content_type: &Option<String>,
        len: usize,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update metadata set uploaded = now(), system_attributes = $1, modified = now(), content_type = $2, content_length = $3 where id = $4")
            .await?;
        let len = len as i64;
        let mut attrs = Map::new();
        attrs.insert(
            "original_file_name".to_owned(),
            Value::String(original_file_name.clone().unwrap_or("--".to_owned())),
        );
        let attrs = Value::Object(attrs);
        txn.execute(&stmt, &[&attrs, content_type, &len, metadata_id])
            .await?;
        if let Some(content_type) = content_type {
            self.ensure_content_type_traits(metadata_id, content_type, &txn)
                .await?;
        }
        txn.commit().await?;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    async fn ensure_content_type_traits(
        &self,
        metadata_id: &Uuid,
        content_type: &str,
        txn: &Transaction<'_>,
    ) -> Result<(), Error> {
        let current_traits = self.get_trait_ids(metadata_id).await?;
        let stmt = txn
            .prepare_cached("select trait_id from trait_content_types where content_type = $1")
            .await?;
        let content_type = content_type.to_owned();
        let result = txn.query(&stmt, &[&content_type]).await?;
        for row in result {
            let content_type = row.get(0);
            if current_traits.contains(&content_type) {
                continue;
            }
            self.add_trait_txn(txn, metadata_id, &content_type).await?;
        }
        Ok(())
    }

    pub async fn set_upload_removed(&self, metadata_id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata set uploaded = null, modified = now(), content_length = 0 where id = $1")
            .await?;
        connection.execute(&stmt, &[&metadata_id]).await?;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    pub async fn add_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        metadata: &MetadataInput,
    ) -> Result<(Uuid, i32, i32), Error> {
        let mut source_id: Option<Uuid> = None;
        let mut source_identifier: Option<String> = None;
        if let Some(source) = &metadata.source {
            source_id = Some(Uuid::parse_str(&source.id)?);
            source_identifier = Some(source.identifier.clone());
        }
        let stmt = txn.prepare("insert into metadata (name, type, content_type, content_length, labels, attributes, source_id, source_identifier, language_tag) values ($1, 'standard', $2, $3, $4, ($5)::jsonb, $6, $7, $8) returning id, version, active_version").await?;
        let labels = metadata.labels.clone().unwrap_or_default();
        let rows = txn
            .query(
                &stmt,
                &[
                    &metadata.name,
                    &metadata.content_type,
                    &metadata.content_length,
                    &labels,
                    &metadata.attributes.as_ref().or(Some(&Value::Null)),
                    &source_id,
                    &source_identifier,
                    &metadata.language_tag,
                ],
            )
            .await?;

        let id: Uuid = rows.first().unwrap().get(0);
        let version: i32 = rows.first().unwrap().get(1);
        let active_version: i32 = rows.first().unwrap().get(2);

        if let Some(trait_ids) = &metadata.trait_ids {
            for trait_id in trait_ids {
                self.add_trait_txn(txn, &id, trait_id).await?
            }
        }

        if let Some(category_ids) = &metadata.category_ids {
            for category_id in category_ids {
                let cid = Uuid::parse_str(category_id)?;
                self.add_category_txn(txn, &id, &cid).await?
            }
        }

        if let Some(profiles) = &metadata.profiles {
            for (index, profile) in profiles.iter().enumerate() {
                let pid = Uuid::parse_str(&profile.profile_id)?;
                self.add_profile_txn(txn, &id, &pid, &profile.relationship, index as i32)
                    .await?
            }
        }

        self.ensure_content_type_traits(&id, &metadata.content_type, txn)
            .await?;

        Ok((id, version, active_version))
    }

    async fn edit_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
        metadata: &MetadataInput,
        source_id: &Option<Uuid>,
        source_identifier: &Option<String>,
    ) -> Result<(), Error> {
        let stmt = txn.prepare("update metadata set name = $1, labels = $2, attributes = $3, language_tag = $4, source_id = $5, source_identifier = $6, content_type = $7, modified = now() where id = $8").await?;
        let labels = metadata.labels.clone().unwrap_or_default();
        txn.query(
            &stmt,
            &[
                &metadata.name,
                &labels,
                &metadata.attributes.as_ref().or(Some(&Value::Null)),
                &metadata.language_tag,
                source_id,
                source_identifier,
                &metadata.content_type,
                &id,
            ],
        )
        .await?;

        if let Some(trait_ids) = &metadata.trait_ids {
            self.delete_traits_txn(txn, id).await?;
            for trait_id in trait_ids {
                self.add_trait_txn(txn, id, trait_id).await?
            }
        }

        if let Some(category_ids) = &metadata.category_ids {
            self.delete_categories_txn(txn, id).await?;
            for category_id in category_ids {
                let cid = Uuid::parse_str(category_id)?;
                self.add_category_txn(txn, id, &cid).await?
            }
        }

        if let Some(profiles) = &metadata.profiles {
            self.delete_profiles_txn(txn, id).await?;
            for (index, profile) in profiles.iter().enumerate() {
                let pid = Uuid::parse_str(&profile.profile_id)?;
                self.add_profile_txn(txn, id, &pid, &profile.relationship, index as i32)
                    .await?
            }
        }

        self.ensure_content_type_traits(id, &metadata.content_type, txn)
            .await?;

        Ok(())
    }

    async fn add_profile_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
        profile_id: &Uuid,
        relationship: &String,
        sort: i32,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare("insert into metadata_profiles (metadata_id, profile_id, relationship, sort) values ($1, $2, $3, $4)")
            .await?;
        txn.execute(&stmt, &[id, profile_id, relationship, &sort])
            .await?;
        Ok(())
    }

    async fn delete_profiles_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare("delete from metadata_profiles where metadata_id = $1")
            .await?;
        txn.execute(&stmt, &[id]).await?;
        Ok(())
    }

    pub async fn delete_trait(&self, id: &Uuid, trait_id: &String) -> Result<(), Error> {
        let conn = self.pool.get().await?;
        let stmt = conn
            .prepare("delete from metadata_traits where metadata_id = $1 and trait_id = $2")
            .await?;
        conn.execute(&stmt, &[id, trait_id]).await?;
        self.on_metadata_changed(id).await?;
        Ok(())
    }

    async fn delete_traits_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare("delete from metadata_traits where metadata_id = $1")
            .await?;
        txn.execute(&stmt, &[id]).await?;
        Ok(())
    }

    async fn add_trait_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
        trait_id: &String,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare("insert into metadata_traits (metadata_id, trait_id) values ($1, $2)")
            .await?;
        txn.execute(&stmt, &[id, trait_id]).await?;
        Ok(())
    }

    pub async fn add_trait(&self, id: &Uuid, trait_id: &String) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("insert into metadata_traits (metadata_id, trait_id) values ($1, $2)")
            .await?;
        connection.execute(&stmt, &[id, trait_id]).await?;
        self.on_metadata_changed(id).await?;
        Ok(())
    }

    async fn add_category_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
        category_id: &Uuid,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare("insert into metadata_categories (metadata_id, category_id) values ($1, $2)")
            .await?;
        txn.execute(&stmt, &[id, category_id]).await?;
        Ok(())
    }

    async fn delete_categories_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare("delete from metadata_categories where metadata_id = $1")
            .await?;
        txn.execute(&stmt, &[id]).await?;
        Ok(())
    }

    pub async fn add_category(&self, id: &Uuid, category_id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "insert into metadata_categories (metadata_id, category_id) values ($1, $2)",
            )
            .await?;
        connection.execute(&stmt, &[id, category_id]).await?;
        self.on_metadata_changed(id).await?;
        Ok(())
    }

    pub async fn delete_category(&self, id: &Uuid, category_id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "delete from metadata_categories where metadata_id = $1 and category_id = $2",
            )
            .await?;
        connection.execute(&stmt, &[id, category_id]).await?;
        self.on_metadata_changed(id).await?;
        Ok(())
    }

    pub async fn add_relationship(
        &self,
        relationship: &MetadataRelationshipInput,
    ) -> Result<(), Error> {
        let id1 = Uuid::parse_str(relationship.id1.as_str())?;
        let id2 = Uuid::parse_str(relationship.id2.as_str())?;
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into metadata_relationships (metadata1_id, metadata2_id, relationship, attributes) values ($1, $2, $3, $4)").await?;
        connection
            .execute(
                &stmt,
                &[
                    &id1,
                    &id2,
                    &relationship.relationship,
                    &relationship.attributes,
                ],
            )
            .await?;
        self.on_metadata_changed(&id1).await?;
        self.on_metadata_changed(&id2).await?;
        Ok(())
    }

    pub async fn get_relationships(&self, id: &Uuid) -> Result<Vec<MetadataRelationship>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare("select * from metadata_relationships where metadata1_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[&id]).await?;
        Ok(rows.iter().map(MetadataRelationship::from).collect())
    }

    pub async fn get_relationship(
        &self,
        id1: &Uuid,
        id2: &Uuid,
    ) -> Result<Option<MetadataRelationship>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare("select * from metadata_relationships where metadata1_id = $1 and metadata2_id = $2").await?;
        let rows = connection.query(&stmt, &[id1, id2]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    pub async fn edit_relationship(
        &self,
        id1: &Uuid,
        id2: &Uuid,
        relationship: &str,
        attributes: Value,
    ) -> Result<(), Error> {
        let relationship = relationship.to_owned();
        let connection = self.pool.get().await?;
        let stmt = connection.prepare("update metadata_relationships set relationship = $1, attributes = $2 where metadata1_id = $3 and metadata2_id = $4 and (relationship = $1 or relationship is null or relationship = '')").await?;
        connection
            .query(&stmt, &[&relationship, &attributes, id1, id2])
            .await?;
        self.on_metadata_changed(id1).await?;
        self.on_metadata_changed(id2).await?;
        Ok(())
    }

    pub async fn delete_relationship(
        &self,
        id1: &Uuid,
        id2: &Uuid,
        relationship: &str,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let relationship = relationship.to_owned();
        let stmt = connection
            .prepare_cached(
                "delete from metadata_relationships where metadata1_id = $1 and metadata2_id = $2 and relationship = $3",
            )
            .await?;
        connection
            .execute(&stmt, &[id1, id2, &relationship])
            .await?;
        self.on_metadata_changed(id1).await?;
        self.on_metadata_changed(id2).await?;
        Ok(())
    }

    pub async fn add_all(
        &self,
        ctx: &BoscaContext,
        metadatas: &mut [MetadataChildInput],
    ) -> Result<Vec<(Uuid, i32, i32)>, Error> {
        let mut conn = self.pool.get().await?;
        let txn = conn.transaction().await?;
        let mut search_documents = Vec::new();
        let ids = self
            .add_all_txn(ctx, &txn, metadatas, &mut search_documents, false, None)
            .await?;
        txn.commit().await?;
        if let Some(storage_system) = ctx.workflow.get_default_search_storage_system().await? {
            index_documents(ctx, &search_documents, &storage_system).await?;
        } else {
            error!("failed to index documents, missing storage system");
        }
        for (id, _, _) in &ids {
            self.on_metadata_changed(id).await?
        }
        Ok(ids)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn add_all_txn(
        &self,
        ctx: &BoscaContext,
        txn: &Transaction<'_>,
        metadatas: &[MetadataChildInput],
        search_documents: &mut Vec<SearchDocumentInput>,
        ignore_permission_check: bool,
        permissions: Option<Vec<Permission>>,
    ) -> Result<Vec<(Uuid, i32, i32)>, Error> {
        let mut new_metadatas = Vec::new();
        for metadata_child in metadatas {
            let metadata = &metadata_child.metadata;
            let has_collection_id = metadata.parent_collection_id.is_some();
            let collection_id = match &metadata.parent_collection_id {
                Some(id) => Uuid::parse_str(id.as_str())?,
                None => Uuid::parse_str("00000000-0000-0000-0000-000000000000")?,
            };
            if !ignore_permission_check {
                ctx.check_collection_action_txn(txn, &collection_id, PermissionAction::Edit)
                    .await?;
            }
            let (id, version, active_version) = self.add_txn(txn, metadata).await?;
            let permissions = if let Some(permissions) = &permissions {
                permissions.clone()
            } else {
                ctx.content
                    .collection_permissions
                    .get_txn(txn, &collection_id)
                    .await?
            };
            for permission in permissions.iter() {
                let metadata_permission = Permission {
                    entity_id: id,
                    group_id: permission.group_id,
                    action: permission.action,
                };
                ctx.content
                    .metadata_permissions
                    .add_metadata_permission_txn(txn, &metadata_permission)
                    .await?
            }
            if has_collection_id {
                ctx.content
                    .collections
                    .add_child_metadata_txn(txn, &collection_id, &id, &metadata_child.attributes)
                    .await?;
            }
            if metadata.index.unwrap_or(true) {
                search_documents.push(SearchDocumentInput {
                    metadata_id: Some(id.to_string()),
                    collection_id: None,
                    profile_id: None,
                    content: "".to_owned(),
                });
            }
            new_metadatas.push((id, version, active_version));
        }
        Ok(new_metadatas)
    }
}
