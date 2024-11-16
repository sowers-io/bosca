use async_graphql::*;
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;
use crate::graphql::content::content::FindAttributeInput;
use crate::models::content::collection::{Collection, CollectionChild, CollectionChildInput, CollectionInput, CollectionType, MetadataChildInput};
use crate::models::content::metadata::{Metadata, MetadataInput};
use crate::models::content::metadata_relationship::{
    MetadataRelationship, MetadataRelationshipInput,
};
use crate::models::content::source::Source;
use crate::models::content::supplementary::{MetadataSupplementary, MetadataSupplementaryInput};
use crate::models::security::permission::{Permission, PermissionAction};
use crate::models::security::principal::Principal;
use crate::models::workflow::traits::Trait;
use crate::security::evaluator::Evaluator;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use log::error;
use postgres_types::ToSql;
use serde_json::{Map, Value};
use tokio_postgres::Statement;
use uuid::Uuid;
use crate::context::BoscaContext;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
use crate::models::content::search::SearchDocumentInput;
use crate::util::RUNNING_BACKGROUND;
use crate::util::storage::index_documents;

#[derive(Clone)]
pub struct ContentDataStore {
    pool: Arc<Pool>,
}

impl ContentDataStore {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    pub async fn get_sources(&self) -> Result<Vec<Source>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from sources order by name asc")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_source_by_id(&self, id: &Uuid) -> Result<Option<Source>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from sources where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    pub async fn get_source_by_name(&self, name: &String) -> Result<Option<Source>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from sources where name = $1")
            .await?;
        let rows = connection.query(&stmt, &[name]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    pub async fn get_traits(&self) -> Result<Vec<Trait>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from traits order by name asc")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        let mut traits = Vec::<Trait>::new();
        let id_stmt = connection
            .prepare_cached("select workflow_id from trait_workflows where trait_id = $1")
            .await?;
        for row in rows.iter() {
            let mut t: Trait = row.into();
            let rows = connection.query(&id_stmt, &[&t.id]).await?;
            t.workflow_ids = rows
                .iter()
                .map(|r| r.get::<&str, String>("workflow_id").to_string())
                .collect();
            traits.push(t);
        }
        Ok(traits)
    }

    pub async fn get_trait(&self, id: &String) -> Result<Option<Trait>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from traits where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        let id_stmt = connection
            .prepare_cached("select workflow_id from trait_workflows where trait_id = $1")
            .await?;
        if let Some(row) = rows.first() {
            let mut t: Trait = row.into();
            let rows = connection.query(&id_stmt, &[&t.id]).await?;
            t.workflow_ids = rows
                .iter()
                .map(|r| r.get::<&str, String>("workflow_id").to_string())
                .collect();
            return Ok(Some(t));
        }
        Ok(None)
    }

    pub async fn has_collection_permission(
        &self,
        collection: &Collection,
        principal: &Principal,
        action: PermissionAction,
    ) -> Result<bool, Error> {
        if action == PermissionAction::View && collection.public && collection.workflow_state_id == "published" {
            return Ok(true);
        }
        if action == PermissionAction::List && collection.public_list && collection.workflow_state_id == "published" {
            return Ok(true);
        }
        let eval = Evaluator::new(self.get_collection_permissions(&collection.id).await?);
        Ok(eval.evaluate(principal, &action))
    }

    pub async fn has_collection_permission_txn(
        &self,
        txn: &Transaction<'_>,
        collection: &Collection,
        principal: &Principal,
        action: PermissionAction,
    ) -> Result<bool, Error> {
        if action == PermissionAction::View && collection.public && collection.workflow_state_id == "published" {
            return Ok(true);
        }
        if action == PermissionAction::List && collection.public_list && collection.workflow_state_id == "published" {
            return Ok(true);
        }
        let eval = Evaluator::new(self.get_collection_permissions_txn(txn, &collection.id).await?);
        Ok(eval.evaluate(principal, &action))
    }

    fn build_find_args<'a>(
        &self,
        query: &str,
        attributes: &'a [FindAttributeInput],
    ) -> (String, Vec<&'a (dyn ToSql + Sync)>) {
        let mut q = query.to_string();
        let mut values = Vec::new();
        let mut pos = 1;
        for i in 0..attributes.len() {
            let attr = attributes.get(i).unwrap();
            if i > 0 {
                q.push_str(" and ");
            }
            q.push_str(
                format!("attributes->>(${}::varchar) = ${}::varchar", pos, pos + 1).as_str(),
            );
            pos += 2;
            values.push(&attr.key as &(dyn ToSql + Sync));
            values.push(&attr.value as &(dyn ToSql + Sync));
        }
        (q.to_string(), values)
    }

    pub async fn find_collections(
        &self,
        attributes: &[FindAttributeInput],
    ) -> Result<Vec<Collection>, Error> {
        let connection = self.pool.get().await?;
        let (query, values) = self.build_find_args("select * from collections where ", attributes);
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_collection(&self, id: &Uuid) -> Result<Option<Collection>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from collections where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    pub async fn get_collections(&self, offset: i64, limit: i64) -> Result<Vec<Collection>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from collections order by name offset $1 limit $2")
            .await?;
        let rows = connection.query(&stmt, &[&offset, &limit]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_collection_trait_ids(&self, id: &Uuid) -> Result<Vec<String>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select trait_id from collection_traits where collection_id = $1")
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

    #[allow(dead_code)]
    pub async fn get_collection_txn(&self, txn: &Transaction<'_>, id: &Uuid) -> Result<Option<Collection>, Error> {
        let stmt = txn.prepare_cached("select * from collections where id = $1").await?;
        let rows = txn.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    pub async fn get_collection_parent_collections(&self, id: &Uuid, offset: i64, limit: i64) -> Result<Vec<Collection>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select c.* from collections c inner join collection_items ci on (c.id = ci.collection_id) where ci.child_collection_id = $1 offset $2 limit $3")
            .await?;
        let rows = connection.query(&stmt, &[id, &offset, &limit]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_collection_permissions(&self, id: &Uuid) -> Result<Vec<Permission>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select collection_id as entity_id, group_id, action from collection_permissions where collection_id = $1").await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_collection_permissions_txn(&self, txn: &Transaction<'_>, id: &Uuid) -> Result<Vec<Permission>, Error> {
        let stmt = txn.prepare_cached("select collection_id as entity_id, group_id, action from collection_permissions where collection_id = $1").await?;
        let rows = txn.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn set_child_item_attributes(
        &self,
        collection_id: &Uuid,
        child_collection_id: Option<Uuid>,
        child_metadata_id: Option<Uuid>,
        attributes: Option<Value>,
    ) -> Result<(), Error> {
        if child_collection_id.is_none() && child_metadata_id.is_none() {
            return Err(Error::new("you must supply either a child collection id or child metadata id"));
        }
        if child_collection_id.is_some() && child_metadata_id.is_some() {
            return Err(Error::new("you can only supply either a child collection id or child metadata id"));
        }
        let connection = self.pool.get().await?;
        if let Some(child_id) = child_collection_id {
            let stmt = connection.prepare_cached("update collection_items set attributes = $1 where collection_id = $2 and child_collection_id = $3").await?;
            connection.execute(&stmt, &[&attributes, collection_id, &child_id]).await?;
        } else if let Some(child_id) = child_metadata_id {
            let stmt = connection.prepare_cached("update collection_items set attributes = $1 where collection_id = $2 and child_metadata_id = $3").await?;
            connection.execute(&stmt, &[&attributes, collection_id, &child_id]).await?;
        }
        Ok(())
    }

    pub async fn set_collection_public(
        &self,
        id: &Uuid,
        public: bool,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update collections set public = $1, modified = now() where id = $2",
            )
            .await?;
        connection.execute(&stmt, &[&public, id]).await?;
        Ok(())
    }

    pub async fn set_collection_public_list(
        &self,
        id: &Uuid,
        public: bool,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update collections set public_list = $1, modified = now() where id = $2",
            )
            .await?;
        connection.execute(&stmt, &[&public, id]).await?;
        Ok(())
    }

    pub async fn add_collection_permission(&self, permission: &Permission) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into collection_permissions (collection_id, group_id, action) values ($1, $2, $3)").await?;
        connection
            .execute(
                &stmt,
                &[
                    &permission.entity_id,
                    &permission.group_id,
                    &permission.action,
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn add_collection_permission_txn(&self, txn: &Transaction<'_>, permission: &Permission) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into collection_permissions (collection_id, group_id, action) values ($1, $2, $3)").await?;
        txn.execute(&stmt, &[&permission.entity_id, &permission.group_id, &permission.action]).await?;
        Ok(())
    }

    pub async fn delete_collection_permission(&self, permission: &Permission) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("delete from collection_permissions where collection_id = $1 and group_id = $2 and action = $3").await?;
        connection
            .execute(
                &stmt,
                &[
                    &permission.entity_id,
                    &permission.group_id,
                    &permission.action,
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn delete_collection(&self, id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("delete from collections where id = $1").await?;
        connection.execute(&stmt, &[id]).await?;
        Ok(())
    }

    fn build_ordering_names(&self, ordering: &[Value], names: &mut Vec<String>) {
        for attr in ordering {
            let a = attr.as_object().unwrap();
            let path = a.get("path").unwrap().as_array().unwrap();
            for p in path {
                names.push(p.as_str().unwrap().to_owned());
            }
        }
    }

    fn build_ordering<'a>(&self, attributes_column: &str, start_index: i32, ordering: &[Value], values: &mut Vec<&'a (dyn ToSql + Sync)>, names: &'a [String]) -> String {
        let mut index = start_index;
        let mut buf = "order by ".to_owned();
        let mut n = 0;
        for (i, attr) in ordering.iter().enumerate() {
            if i > 0 {
                buf.push_str(", ");
            }
            let a = attr.as_object().unwrap();
            let path = a.get("path").unwrap().as_array().unwrap();
            buf.push_str(attributes_column);
            for _ in path {
                let name = names.get(n).unwrap();
                n += 1;
                values.push(name as &(dyn ToSql + Sync));
                buf.push_str(format!("->${}", index).as_str());
                index += 1;
            }
            if a.contains_key("order") {
                buf.push(' ');
                buf.push_str(if a.get("order").unwrap().as_str().unwrap() == "asc" { "asc" } else { "desc" });
            }
        }
        if buf == "order by " {
            return "".to_owned();
        }
        buf
    }

    pub async fn get_collection_children(
        &self,
        collection: &Collection,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<CollectionChild>, Error> {
        let mut values = Vec::new();
        let mut names = Vec::new();
        values.push(&collection.id as &(dyn ToSql + Sync));
        let ordering = match &collection.ordering {
            Some(Value::Array(ordering)) => {
                self.build_ordering_names(ordering, &mut names);
                self.build_ordering("attributes", 2, ordering, &mut values, &names)
            }
            _ => String::new()
        };
        let mut query = "select child_collection_id, child_metadata_id, attributes from collection_items where collection_id = $1 ".to_owned();
        if !ordering.is_empty() {
            query.push_str(ordering.as_str());
        }
        query.push_str(format!(" offset ${} limit ${}", values.len() + 1, values.len() + 2).as_str());
        values.push(&offset as &(dyn ToSql + Sync));
        values.push(&limit as &(dyn ToSql + Sync));
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_collection_child_collections(
        &self,
        collection: &Collection,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Collection>, Error> {
        let mut values = Vec::new();
        let mut names = Vec::new();
        values.push(&collection.id as &(dyn ToSql + Sync));
        let ordering = match &collection.ordering {
            Some(Value::Array(ordering)) => {
                self.build_ordering_names(ordering, &mut names);
                self.build_ordering("ci.attributes", 2, ordering, &mut values, &names)
            }
            _ => String::new()
        };
        let mut query = "select c.*, ci.attributes as item_attributes from collections c inner join collection_items ci on (ci.child_collection_id = c.id and ci.collection_id = $1) ".to_owned();
        if ordering.is_empty() {
            query.push_str("order by name asc");
        } else {
            query.push_str(ordering.as_str());
        }
        query.push_str(format!(" offset ${} limit ${}", values.len() + 1, values.len() + 2).as_str());
        values.push(&offset as &(dyn ToSql + Sync));
        values.push(&limit as &(dyn ToSql + Sync));
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_collection_child_metadata(
        &self,
        collection: &Collection,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Metadata>, Error> {
        let mut values = Vec::new();
        let mut names = Vec::new();
        values.push(&collection.id as &(dyn ToSql + Sync));
        let ordering = match &collection.ordering {
            Some(Value::Array(ordering)) => {
                self.build_ordering_names(ordering, &mut names);
                self.build_ordering("ci.attributes", 2, ordering, &mut values, &names)
            }
            _ => String::new()
        };
        let mut query = "select m.*, ci.attributes as item_attributes from metadata m inner join collection_items ci on (ci.child_metadata_id = m.id and ci.collection_id = $1) ".to_owned();
        if ordering.is_empty() {
            query.push_str("order by name asc");
        } else {
            query.push_str(ordering.as_str());
        }
        query.push_str(format!(" offset ${} limit ${}", values.len() + 1, values.len() + 2).as_str());
        values.push(&offset as &(dyn ToSql + Sync));
        values.push(&limit as &(dyn ToSql + Sync));
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn add_collection(&self, collection: &CollectionInput) -> Result<Uuid, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;

        match self.add_collection_txn(&txn, collection).await {
            Ok(value) => {
                txn.commit().await?;
                Ok(value)
            }
            Err(err) => {
                txn.rollback().await?;
                Err(err)
            }
        }
    }

    pub async fn edit_collection(&self, id: &Uuid, collection: &CollectionInput) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;

        match self.edit_collection_txn(&txn, id, collection).await {
            Ok(value) => {
                txn.commit().await?;
                Ok(value)
            }
            Err(err) => {
                txn.rollback().await?;
                Err(err)
            }
        }
    }

    pub async fn add_collection_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        collection: &CollectionInput,
    ) -> Result<Uuid, Error> {
        let stmt: Statement = if collection.collection_type.unwrap_or(CollectionType::Folder) == CollectionType::Root {
            txn.prepare("insert into collections (id, name, description, type, labels, attributes, ordering) values ('00000000-0000-0000-0000-000000000000', $1, $2, $3, $4, $5, $6) returning id").await?
        } else {
            txn.prepare("insert into collections (name, description, type, labels, attributes, ordering) values ($1, $2, $3, $4, $5, $6) returning id").await?
        };
        let labels = collection.labels.clone().unwrap_or_default();
        let rows = txn
            .query(
                &stmt,
                &[
                    &collection.name,
                    &collection.description,
                    &collection.collection_type.unwrap_or(CollectionType::Folder),
                    &labels,
                    &collection.attributes.as_ref().or(Some(&Value::Null)),
                    &collection.ordering.as_ref().or(Some(&Value::Null)),
                ],
            )
            .await?;
        Ok(rows.first().unwrap().get(0))
    }

    async fn edit_collection_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
        collection: &CollectionInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare("update collections set name = $1, description = $2, type = $3, labels = $4, attributes = $5, ordering = $6 where id = $7").await?;
        let labels = collection.labels.clone().unwrap_or_default();
        txn.execute(
            &stmt,
            &[
                &collection.name,
                &collection.description,
                &collection.collection_type.unwrap_or(CollectionType::Folder),
                &labels,
                &collection.attributes.as_ref().or(Some(&Value::Null)),
                &collection.ordering.as_ref().or(Some(&Value::Null)),
                id,
            ],
        )
            .await?;
        Ok(())
    }

    pub async fn add_child_collection(&self, id: &Uuid, collection_id: &Uuid, attributes: &Option<Value>) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "insert into collection_items (collection_id, child_collection_id, attributes) values ($1, $2, $3)",
            )
            .await?;
        connection.execute(&stmt, &[id, collection_id, attributes]).await?;
        Ok(())
    }

    pub async fn add_child_collection_txn(&self, txn: &Transaction<'_>, id: &Uuid, collection_id: &Uuid, attributes: &Option<Value>) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into collection_items (collection_id, child_collection_id, attributes) values ($1, $2, $3)").await?;
        txn.execute(&stmt, &[id, collection_id, attributes]).await?;
        Ok(())
    }

    pub async fn add_child_metadata(&self, id: &Uuid, metadata_id: &Uuid, attributes: &Option<Value>) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "insert into collection_items (collection_id, child_metadata_id, attributes) values ($1, $2, $3)",
            )
            .await?;
        connection.execute(&stmt, &[id, metadata_id, attributes]).await?;
        Ok(())
    }

    pub async fn add_child_metadata_txn(&self, txn: &Transaction<'_>, id: &Uuid, metadata_id: &Uuid, attributes: &Option<Value>) -> Result<(), Error> {
        let stmt = txn
            .prepare_cached(
                "insert into collection_items (collection_id, child_metadata_id, attributes) values ($1, $2, $3)",
            )
            .await?;
        txn.execute(&stmt, &[id, metadata_id, attributes]).await?;
        Ok(())
    }

    pub async fn remove_child_collection(&self, id: &Uuid, collection_id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "delete from collection_items where collection_id = $1 and child_collection_id = $2",
            )
            .await?;
        connection.execute(&stmt, &[id, collection_id]).await?;
        Ok(())
    }

    pub async fn remove_child_metadata(&self, id: &Uuid, metadata_id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "delete from collection_items where collection_id = $1 and child_metadata_id = $2",
            )
            .await?;
        connection.execute(&stmt, &[id, metadata_id]).await?;
        Ok(())
    }

    pub async fn has_metadata_permission(
        &self,
        metadata: &Metadata,
        principal: &Principal,
        action: PermissionAction,
    ) -> Result<bool, Error> {
        if action == PermissionAction::View && metadata.public && metadata.workflow_state_id == "published" {
            return Ok(true);
        }
        let eval = Evaluator::new(self.get_metadata_permissions(&metadata.id).await?);
        Ok(eval.evaluate(principal, &action))
    }

    pub async fn find_metadata(
        &self,
        attributes: &[FindAttributeInput],
    ) -> Result<Vec<Metadata>, Error> {
        let connection = self.pool.get().await?;
        let (query, values) = self.build_find_args("select * from metadata where ", attributes);
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_metadata(&self, id: &Uuid) -> Result<Option<Metadata>, Error> {
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

    pub async fn get_metadatas(&self, offset: i64, limit: i64) -> Result<Vec<Metadata>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from metadata order by name offset $1 limit $2")
            .await?;
        let rows = connection.query(&stmt, &[&offset, &limit]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_metadata_by_version(&self, id: &Uuid, version: i32) -> Result<Option<Metadata>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from metadata where id = $1 and version = $2")
            .await?;
        let rows = connection.query(&stmt, &[id, &version]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    pub async fn get_metadata_parent_collection_ids(&self, id: &Uuid, offset: i64, limit: i64) -> Result<Vec<Uuid>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select collection_id from collection_items where child_metadata_id = $1 offset $2 limit $3")
            .await?;
        let rows = connection.query(&stmt, &[id, &offset, &limit]).await?;
        Ok(rows.iter().map(|r| r.get("collection_id")).collect())
    }

    #[allow(dead_code)]
    pub async fn get_metadata_parent_collections(&self, id: &Uuid, offset: i64, limit: i64) -> Result<Vec<Collection>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select c.* from collections c inner join collection_items ci on (c.id = ci.collection_id) where ci.child_metadata_id = $1 offset $2 limit $3")
            .await?;
        let rows = connection.query(&stmt, &[id, &offset, &limit]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_metadata_trait_ids(&self, id: &Uuid) -> Result<Vec<String>, Error> {
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

    pub async fn get_metadata_supplementary(
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

    pub async fn get_metadata_permissions(&self, id: &Uuid) -> Result<Vec<Permission>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select metadata_id as entity_id, group_id, action from metadata_permissions where metadata_id = $1").await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_metadata_supplementaries(
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

    pub async fn add_metadata_supplementary(
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
        Ok(())
    }

    pub async fn set_metadata_supplementary_uploaded(
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
        Ok(())
    }

    pub async fn set_metadata_ready(
        &self,
        id: &Uuid,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update metadata set ready = now() where id = $1",
            )
            .await?;
        connection.execute(&stmt, &[id]).await?;
        Ok(())
    }

    pub async fn add_metadata_plan(
        &self,
        id: &Uuid,
        plan_id: i64,
        queue: &String,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "insert into metadata_workflow_plans (id, plan_id, queue) values ($1, $2, $3)",
            )
            .await?;
        let job_id = plan_id.to_string();
        connection.execute(&stmt, &[id, &job_id, queue]).await?;
        Ok(())
    }

    pub async fn get_metadata_plans(&self, id: &Uuid) -> Result<Vec<(String, String)>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select plan_id, queue from metadata_workflow_plans where id = $1")
            .await?;
        let results = connection.query(&stmt, &[id]).await?;
        Ok(results
            .iter()
            .map(|r| {
                let plan_id: String = r.get("plan_id");
                let queue: String = r.get("queue");
                (plan_id, queue)
            })
            .collect())
    }

    pub async fn get_collection_plans(&self, id: &Uuid) -> Result<Vec<(String, String)>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select plan_id, queue from collection_workflow_plans where id = $1")
            .await?;
        let results = connection.query(&stmt, &[id]).await?;
        Ok(results
            .iter()
            .map(|r| {
                let plan_id: String = r.get("plan_id");
                let queue: String = r.get("queue");
                (plan_id, queue)
            })
            .collect())
    }

    pub async fn delete_metadata(&self, metadata_id: &Uuid) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("delete from metadata where id = $1")
            .await?;
        txn.execute(&stmt, &[&metadata_id]).await?;
        let stmt = txn
            .prepare_cached("delete from metadata_versions where id = $1")
            .await?;
        txn.execute(&stmt, &[&metadata_id]).await?;
        txn.commit().await?;
        Ok(())
    }

    pub async fn delete_metadata_supplementary(
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
        Ok(())
    }

    pub async fn set_metadata_public(
        &self,
        id: &Uuid,
        public: bool,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update metadata set public = $1, modified = now() where id = $2",
            )
            .await?;
        connection.execute(&stmt, &[&public, id]).await?;
        Ok(())
    }

    pub async fn set_metadata_public_content(
        &self,
        id: &Uuid,
        public: bool,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update metadata set public_content = $1, modified = now() where id = $2",
            )
            .await?;
        connection.execute(&stmt, &[&public, id]).await?;
        Ok(())
    }

    pub async fn set_metadata_public_supplementary(
        &self,
        id: &Uuid,
        public: bool,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update metadata set public_supplementary = $1, modified = now() where id = $2",
            )
            .await?;
        connection.execute(&stmt, &[&public, id]).await?;
        Ok(())
    }

    pub async fn add_metadata_permission(&self, permission: &Permission) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into metadata_permissions (metadata_id, group_id, action) values ($1, $2, $3)").await?;
        connection
            .execute(
                &stmt,
                &[
                    &permission.entity_id,
                    &permission.group_id,
                    &permission.action,
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn add_metadata_permission_txn(&self, txn: &Transaction<'_>, permission: &Permission) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into metadata_permissions (metadata_id, group_id, action) values ($1, $2, $3)").await?;
        txn
            .execute(
                &stmt,
                &[
                    &permission.entity_id,
                    &permission.group_id,
                    &permission.action,
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn delete_metadata_permission(&self, permission: &Permission) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("delete from metadata_permissions where metadata_id = $1 and group_id = $2 and action = $3").await?;
        connection
            .execute(
                &stmt,
                &[
                    &permission.entity_id,
                    &permission.group_id,
                    &permission.action,
                ],
            )
            .await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn add_metadata(&self, metadata: &MetadataInput) -> Result<(Uuid, i32), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;

        match self
            .add_metadata_txn(&txn, metadata)
            .await
        {
            Ok(value) => {
                txn.commit().await?;
                Ok(value)
            }
            Err(err) => {
                txn.rollback().await?;
                Err(err)
            }
        }
    }

    pub async fn edit_metadata(&self, id: &Uuid, metadata: &MetadataInput) -> Result<(), Error> {
        let mut source_id: Option<Uuid> = None;
        let mut source_identifier: Option<String> = None;
        if let Some(source) = &metadata.source {
            source_id = Some(Uuid::parse_str(&source.id)?);
            source_identifier = Some(source.identifier.clone());
        }
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;

        match self
            .edit_metadata_txn(&txn, id, metadata, &source_id, &source_identifier)
            .await
        {
            Ok(value) => {
                txn.commit().await?;
                Ok(value)
            }
            Err(err) => {
                txn.rollback().await?;
                Err(err)
            }
        }
    }

    pub async fn set_metadata_attributes(&self, metadata_id: &Uuid, attributes: Value) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata set attributes = $1, modified = now() where id = $2")
            .await?;
        connection.execute(&stmt, &[&attributes, &metadata_id]).await?;
        Ok(())
    }

    pub async fn set_metadata_system_attributes(&self, metadata_id: &Uuid, attributes: Value) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata set system_attributes = $1, modified = now() where id = $2")
            .await?;
        connection.execute(&stmt, &[&attributes, &metadata_id]).await?;
        Ok(())
    }

    pub async fn set_metadata_uploaded(&self, metadata_id: &Uuid, original_file_name: &Option<String>, content_type: &Option<String>, len: usize) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata set uploaded = now(), system_attributes = $1, modified = now(), content_type = $2, content_length = $3 where id = $4")
            .await?;
        let len = len as i64;
        let mut attrs = Map::new();
        attrs.insert("original_file_name".to_owned(), Value::String(original_file_name.clone().unwrap_or("--".to_owned())));
        let attrs = Value::Object(attrs);
        connection.execute(&stmt, &[&attrs, content_type, &len, metadata_id]).await?;
        Ok(())
    }

    pub async fn set_metadata_upload_removed(&self, metadata_id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata set uploaded = null, modified = now(), content_length = 0 where id = $1")
            .await?;
        connection.execute(&stmt, &[&metadata_id]).await?;
        Ok(())
    }

    pub async fn add_metadata_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        metadata: &MetadataInput,
    ) -> Result<(Uuid, i32), Error> {
        let mut source_id: Option<Uuid> = None;
        let mut source_identifier: Option<String> = None;
        if let Some(source) = &metadata.source {
            source_id = Some(Uuid::parse_str(&source.id)?);
            source_identifier = Some(source.identifier.clone());
        }
        let stmt = txn.prepare("insert into metadata (name, type, content_type, content_length, labels, attributes, source_id, source_identifier, language_tag) values ($1, 'standard', $2, $3, $4, ($5)::jsonb, $6, $7, $8) returning id, version").await?;
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

        if let Some(trait_ids) = &metadata.trait_ids {
            for trait_id in trait_ids {
                self.add_metadata_trait_txn(txn, &id, trait_id).await?
            }
        }

        if let Some(category_ids) = &metadata.category_ids {
            for category_id in category_ids {
                let cid = Uuid::parse_str(category_id)?;
                self.add_metadata_category_txn(txn, &id, &cid).await?
            }
        }

        Ok((id, version))
    }

    async fn edit_metadata_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
        metadata: &MetadataInput,
        source_id: &Option<Uuid>,
        source_identifier: &Option<String>,
    ) -> Result<(), Error> {
        if let Some(trait_ids) = &metadata.trait_ids {
            if !trait_ids.is_empty() {
                return Err(Error::new("cannot bulk set trait ids after metadata is created"));
            }
        }

        if let Some(category_ids) = &metadata.category_ids {
            if !category_ids.is_empty() {
                return Err(Error::new("cannot bulk set category ids after metadata is created"));
            }
        }

        let stmt = txn.prepare("update metadata set name = $1, labels = $2, attributes = $3, language_tag = $4, source_id = $5, source_identifier = $6 where id = $7").await?;
        let labels = metadata.labels.clone().unwrap_or_default();
        txn
            .query(
                &stmt,
                &[
                    &metadata.name,
                    &labels,
                    &metadata.attributes.as_ref().or(Some(&Value::Null)),
                    &metadata.language_tag,
                    source_id,
                    source_identifier,
                    &id
                ],
            )
            .await?;

        Ok(())
    }

    pub async fn delete_metadata_trait(
        &self,
        id: &Uuid,
        trait_id: &String,
    ) -> Result<(), Error> {
        let conn = self.pool.get().await?;
        let stmt = conn
            .prepare("delete from metadata_traits where metadata_id = $1 and trait_id = $2")
            .await?;
        conn.execute(&stmt, &[id, trait_id]).await?;
        Ok(())
    }

    #[allow(dead_code)]
    async fn delete_metadata_trait_txn<'a>(
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

    async fn add_metadata_trait_txn<'a>(
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

    pub async fn add_metadata_trait(&self, id: &Uuid, trait_id: &String) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("insert into metadata_traits (metadata_id, trait_id) values ($1, $2)")
            .await?;
        connection.execute(&stmt, &[id, trait_id]).await?;
        Ok(())
    }

    #[allow(dead_code)]
    async fn delete_metadata_category_txn<'a>(
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

    async fn add_metadata_category_txn<'a>(
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

    pub async fn add_metadata_category(&self, id: &Uuid, category_id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "insert into metadata_categories (metadata_id, category_id) values ($1, $2)",
            )
            .await?;
        connection.execute(&stmt, &[id, category_id]).await?;
        Ok(())
    }

    pub async fn delete_metadata_category(
        &self,
        id: &Uuid,
        category_id: &Uuid,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "delete from metadata_categories where metadata_id = $1 and category_id = $2",
            )
            .await?;
        connection.execute(&stmt, &[id, category_id]).await?;
        Ok(())
    }

    pub async fn add_metadata_relationship(
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
        Ok(())
    }

    pub async fn get_metadata_relationships(
        &self,
        id: &Uuid,
    ) -> Result<Vec<MetadataRelationship>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare("select * from metadata_relationships where metadata1_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[&id]).await?;
        Ok(rows.iter().map(MetadataRelationship::from).collect())
    }

    pub async fn get_metadata_relationship(
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

    pub async fn edit_metadata_relationship(
        &self,
        id1: &Uuid,
        id2: &Uuid,
        relationship: &str,
        attributes: Value,
    ) -> Result<(), Error> {
        let relationship = relationship.to_owned();
        let connection = self.pool.get().await?;
        let stmt = connection.prepare("update metadata_relationships set relationship = $1, attributes = $2 where metadata1_id = $3 and metadata2_id = $4 and (relationship = $1 or relationship is null or relationship = '')").await?;
        connection.query(&stmt, &[&relationship, &attributes, id1, id2]).await?;
        Ok(())
    }

    pub async fn delete_metadata_relationship(&self, id1: &Uuid, id2: &Uuid, relationship: &str) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let relationship = relationship.to_owned();
        let stmt = connection
            .prepare_cached(
                "delete from metadata_relationships where metadata1_id = $1 and metadata2_id = $2 and relationship = $3",
            )
            .await?;
        connection.execute(&stmt, &[id1, id2, &relationship]).await?;
        Ok(())
    }

    pub async fn set_metadata_workflow_state(
        &self,
        principal: &Principal,
        metadata: &Metadata,
        to_state_id: &str,
        status: &str,
        success: bool,
        complete: bool,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let state = to_state_id.to_owned();
        let status = status.to_owned();
        let stmt = txn.prepare_cached("insert into metadata_workflow_transition_history (metadata_id, from_state_id, to_state_id, principal, status, success, complete) values ($1, $2, $3, $4, $5, $6, $7)").await?;
        txn.execute(
            &stmt,
            &[
                &metadata.id,
                &metadata.workflow_state_id,
                &state,
                &principal.id,
                &status,
                &success,
                &complete,
            ],
        )
            .await?;
        if !success {
            let stmt = txn
                .prepare("update metadata set workflow_state_pending_id = null where id = $1")
                .await?;
            txn.execute(&stmt, &[&metadata.id]).await?;
        } else if complete {
            let stmt = txn.prepare("update metadata set workflow_state_id = $1, workflow_state_pending_id = null where id = $2").await?;
            txn.execute(&stmt, &[&state, &metadata.id]).await?;
        } else {
            let stmt = txn
                .prepare("update metadata set workflow_state_pending_id = $1 where id = $2")
                .await?;
            txn.execute(&stmt, &[&state, &metadata.id]).await?;
        }
        txn.commit().await?;
        Ok(())
    }

    pub async fn set_collection_attributes(&self, collection_id: &Uuid, attributes: Value) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update collections set attributes = $1, modified = now() where id = $2")
            .await?;
        connection.execute(&stmt, &[&attributes, &collection_id]).await?;
        Ok(())
    }

    pub async fn set_collection_ordering(&self, collection_id: &Uuid, ordering: Value) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update collections set ordering = $1, modified = now() where id = $2")
            .await?;
        connection.execute(&stmt, &[&ordering, &collection_id]).await?;
        Ok(())
    }

    pub async fn set_collection_ready(
        &self,
        id: &Uuid,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update collections set ready = now() where id = $1",
            )
            .await?;
        connection.execute(&stmt, &[id]).await?;
        Ok(())
    }

    pub async fn add_collection_plan(
        &self,
        id: &Uuid,
        plan_id: i64,
        queue: &String,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "insert into collection_workflow_plans (id, plan_id, queue) values ($1, $2, $3)",
            )
            .await?;
        let job_id = plan_id.to_string();
        connection.execute(&stmt, &[id, &job_id, queue]).await?;
        Ok(())
    }

    pub async fn set_collection_workflow_state(
        &self,
        principal: &Principal,
        collection: &Collection,
        to_state_id: &str,
        status: &str,
        success: bool,
        complete: bool,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let state = to_state_id.to_owned();
        let status = status.to_owned();
        let stmt = txn.prepare_cached("insert into collection_workflow_transition_history (collection_id, from_state_id, to_state_id, principal, status, success, complete) values ($1, $2, $3, $4, $5, $6, $7)").await?;
        txn.execute(
            &stmt,
            &[
                &collection.id,
                &collection.workflow_state_id,
                &state,
                &principal.id,
                &status,
                &success,
                &complete,
            ],
        )
            .await?;
        if !success {
            let stmt = txn
                .prepare("update collections set workflow_state_pending_id = null where id = $1")
                .await?;
            txn.execute(&stmt, &[&collection.id]).await?;
        } else if complete {
            let stmt = txn.prepare("update collections set workflow_state_id = $1, workflow_state_pending_id = null where id = $2").await?;
            txn.execute(&stmt, &[&state, &collection.id]).await?;
        } else {
            let stmt = txn
                .prepare("update collections set workflow_state_pending_id = $1 where id = $2")
                .await?;
            txn.execute(&stmt, &[&state, &collection.id]).await?;
        }
        txn.commit().await?;
        Ok(())
    }

    pub async fn add_collections(&self, ctx: &BoscaContext, collections: &mut [CollectionChildInput]) -> Result<Vec<Uuid>, Error> {
        let mut search_documents = Vec::new();
        let mut ready_collection_ids = Vec::new();
        let mut ready_metadata_ids = Vec::new();
        let ids = {
            let mut conn = self.pool.get().await?;
            let txn = conn.transaction().await?;
            let ids = self.add_collections_txn(ctx, &txn, collections, &mut ready_collection_ids, &mut ready_metadata_ids, &mut search_documents, false, None).await?;
            txn.commit().await?;
            ids
        };
        let new_ctx = ctx.clone();
        tokio::spawn(async move {
            match new_ctx.workflow.get_default_search_storage_system().await {
                Ok(storage_system) => {
                    if let Err(err) = index_documents(&new_ctx, &search_documents, &storage_system).await {
                        error!("failed to index documents: {}", err.message);
                    }
                }
                Err(err) => {
                    error!("failed to get storage system to index documents: {}", err.message);
                }
            }
        });
        for id in ready_collection_ids {
            let new_ctx = ctx.clone();
            tokio::spawn(async move {
                RUNNING_BACKGROUND.fetch_add(1, Relaxed);
                match new_ctx.content.get_collection(&id).await {
                    Ok(Some(collection)) => {
                        if let Err(e) = new_ctx.content.set_collection_ready_and_enqueue(&new_ctx, &collection, None).await {
                            error!("failed to queue all collections: {}", e.message);
                        }
                    }
                    Ok(None) => {
                        error!("failed to get collection: not found");
                    }
                    Err(e) => {
                        error!("failed to get collection: {}", e.message);
                    }
                }
                RUNNING_BACKGROUND.fetch_add(-1, Relaxed);
            });
        }
        for (id, version) in ready_metadata_ids {
            let new_ctx = ctx.clone();
            tokio::spawn(async move {
                RUNNING_BACKGROUND.fetch_add(1, Relaxed);
                match new_ctx.content.get_metadata_by_version(&id, version).await {
                    Ok(Some(metadata)) => {
                        if let Err(e) = new_ctx.content.set_metadata_ready_and_enqueue(&new_ctx, &metadata, None).await {
                            error!("failed to queue all metadata: {}", e.message);
                        }
                    }
                    Ok(None) => {
                        error!("failed to get metadata: not found");
                    }
                    Err(e) => {
                        error!("failed to get metadata: {}", e.message);
                    }
                }
                RUNNING_BACKGROUND.fetch_add(-1, Relaxed);
            });
        }
        Ok(ids)
    }

    #[allow(clippy::too_many_arguments)]
    async fn add_collections_txn(&self, ctx: &BoscaContext, txn: &Transaction<'_>, collections: &mut [CollectionChildInput], ready_collection_ids: &mut Vec<Uuid>, ready_metadata_ids: &mut Vec<(Uuid, i32)>, search_documents: &mut Vec<SearchDocumentInput>, ignore_permission_check: bool, permissions: Option<Vec<Permission>>) -> Result<Vec<Uuid>, Error> {
        let mut new_collections = Vec::new();
        for collection_child in collections.iter_mut() {
            let collection = &mut collection_child.collection;
            let has_collection_id = collection.parent_collection_id.is_some();
            let collection_id = match &collection.parent_collection_id {
                Some(id) => Uuid::parse_str(id.as_str())?,
                None => Uuid::parse_str("00000000-0000-0000-0000-000000000000")?,
            };
            if !ignore_permission_check {
                ctx.check_collection_action_txn(txn, &collection_id, PermissionAction::Edit).await?;
            }
            let id = self.add_collection_txn(txn, collection).await?;
            let permissions = if let Some(permissions) = &permissions {
                permissions.clone()
            } else {
                self.get_collection_permissions_txn(txn, &collection_id).await?
            };
            for permission in permissions.iter() {
                let collection_permission = Permission {
                    entity_id: id,
                    group_id: permission.group_id,
                    action: permission.action,
                };
                self.add_collection_permission_txn(txn, &collection_permission).await?
            }
            if has_collection_id {
                self.add_child_collection_txn(txn, &collection_id, &id, &collection_child.attributes).await?;
            }
            if let Some(children) = &mut collection.collections {
                for child in children.iter_mut() {
                    child.collection.parent_collection_id = Some(id.to_string());
                }
                Box::pin(self.add_collections_txn(ctx, txn, children, ready_collection_ids, ready_metadata_ids, search_documents, true, Some(permissions.clone()))).await?;
            }
            if let Some(children) = &mut collection.metadata {
                for child in children.iter_mut() {
                    child.metadata.parent_collection_id = Some(id.to_string());
                }
                self.add_metadatas_txn(ctx, txn, children, ready_metadata_ids, search_documents, true, Some(permissions.clone())).await?;
            }
            if collection.index.unwrap_or(true) {
                search_documents.push(SearchDocumentInput {
                    collection_id: Some(id.to_string()),
                    metadata_id: None,
                    content: "".to_owned(),
                });
            }
            if collection.ready.unwrap_or(false) {
                ready_collection_ids.push(id);
            }
            new_collections.push(id);
        }
        Ok(new_collections)
    }

    pub async fn add_metadatas(&self, ctx: &BoscaContext, metadatas: &mut [MetadataChildInput]) -> Result<Vec<(Uuid, i32)>, Error> {
        let mut conn = self.pool.get().await?;
        let txn = conn.transaction().await?;
        let mut ready_metadata_ids = Vec::new();
        let mut search_documents = Vec::new();
        let ids = self.add_metadatas_txn(ctx, &txn, metadatas, &mut ready_metadata_ids, &mut search_documents, false, None).await?;
        txn.commit().await?;
        let storage_system = ctx.workflow.get_default_search_storage_system().await?;
        index_documents(ctx, &search_documents, &storage_system).await?;
        for (id, version) in ready_metadata_ids {
            if let Some(metadata) = self.get_metadata_by_version(&id, version).await? {
                self.set_metadata_ready_and_enqueue(ctx, &metadata, None).await?;
            }
        }
        Ok(ids)
    }

    #[allow(clippy::too_many_arguments)]
    async fn add_metadatas_txn(&self, ctx: &BoscaContext, txn: &Transaction<'_>, metadatas: &[MetadataChildInput], ready_metadata_ids: &mut Vec<(Uuid, i32)>, search_documents: &mut Vec<SearchDocumentInput>, ignore_permission_check: bool, permissions: Option<Vec<Permission>>) -> Result<Vec<(Uuid, i32)>, Error> {
        let mut new_metadatas = Vec::new();
        for metadata_child in metadatas {
            let metadata = &metadata_child.metadata;
            let has_collection_id = metadata.parent_collection_id.is_some();
            let collection_id = match &metadata.parent_collection_id {
                Some(id) => Uuid::parse_str(id.as_str())?,
                None => Uuid::parse_str("00000000-0000-0000-0000-000000000000")?,
            };
            if !ignore_permission_check {
                ctx.check_collection_action_txn(txn, &collection_id, PermissionAction::Edit).await?;
            }
            let (id, version) = self.add_metadata_txn(txn, metadata).await?;
            let permissions = if let Some(permissions) = &permissions {
                permissions.clone()
            } else {
                self.get_collection_permissions_txn(txn, &collection_id).await?
            };
            for permission in permissions.iter() {
                let metadata_permission = Permission {
                    entity_id: id,
                    group_id: permission.group_id,
                    action: permission.action,
                };
                self.add_metadata_permission_txn(txn, &metadata_permission).await?
            }
            if has_collection_id {
                self.add_child_metadata_txn(txn, &collection_id, &id, &metadata_child.attributes).await?;
            }
            if metadata.index.unwrap_or(true) {
                search_documents.push(SearchDocumentInput {
                    metadata_id: Some(id.to_string()),
                    collection_id: None,
                    content: "".to_owned(),
                });
            }
            if metadata.ready.unwrap_or(false) {
                ready_metadata_ids.push((id, version));
            }
            new_metadatas.push((id, version));
        }
        Ok(new_metadatas)
    }

    pub async fn set_collection_ready_and_enqueue(&self, ctx: &BoscaContext, collection: &Collection, configurations: Option<Vec<WorkflowConfigurationInput>>) -> Result<(), Error> {
        let datasource = &ctx.content;
        let workflow = &ctx.workflow;
        let process_id = "collection.process".to_owned();
        self.set_collection_workflow_state(&ctx.principal, collection, "draft", "move to draft during set to ready", true, false).await?;
        let plan = workflow.enqueue_collection_workflow(&process_id, &collection.id, configurations.as_ref(), None).await?;
        datasource.set_collection_ready(&collection.id).await?;
        datasource.add_collection_plan(&collection.id, plan.plan_id, &plan.workflow.queue).await?;
        Ok(())
    }

    pub async fn set_metadata_ready_and_enqueue(&self, ctx: &BoscaContext, metadata: &Metadata, configurations: Option<Vec<WorkflowConfigurationInput>>) -> Result<(), Error> {
        let datasource = &ctx.content;
        let workflow = &ctx.workflow;
        let process_id = "metadata.process".to_owned();
        self.set_metadata_workflow_state(&ctx.principal, metadata, "draft", "move to draft during set to ready", true, false).await?;
        let plan = workflow.enqueue_metadata_workflow(&process_id, &metadata.id, &metadata.version, configurations.as_ref(), None).await?;
        datasource.set_metadata_ready(&metadata.id).await?;
        datasource.add_metadata_plan(&metadata.id, plan.plan_id, &plan.workflow.queue).await?;
        Ok(())
    }
}
