use crate::context::BoscaContext;
use crate::datastores::content::util::{build_find_args, build_ordering, build_ordering_names};
use crate::datastores::notifier::Notifier;
use crate::graphql::content::content::FindAttributeInput;
use crate::models::content::collection::{
    Collection, CollectionChild, CollectionChildInput, CollectionInput, CollectionType,
};
use crate::models::content::metadata::Metadata;
use crate::models::content::search::SearchDocumentInput;
use crate::models::security::permission::{Permission, PermissionAction};
use crate::util::storage::index_documents;
use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use log::error;
use postgres_types::ToSql;
use serde_json::Value;
use std::sync::Arc;
use tokio_postgres::Statement;
use uuid::Uuid;

#[derive(Clone)]
pub struct CollectionsDataStore {
    pool: Arc<Pool>,
    notifier: Arc<Notifier>,
}

impl CollectionsDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

    async fn on_collection_changed(&self, id: &Uuid) -> Result<(), Error> {
        if let Err(e) = self.notifier.collection_changed(id).await {
            error!("Failed to notify collection changes: {:?}", e);
        }
        Ok(())
    }

    async fn on_metadata_changed(&self, id: &Uuid) -> Result<(), Error> {
        if let Err(e) = self.notifier.metadata_changed(id).await {
            error!("Failed to notify metadata changes: {:?}", e);
        }
        Ok(())
    }

    pub async fn get_slug(&self, id: &Uuid) -> Result<String, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select slug from slugs where collection_id = $1")
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
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Collection>, Error> {
        let mut limit = limit;
        if limit > 10000 {
            // TODO: come up with a reasonable limit... or make it configurable somewhere.
            limit = 10000;
        }
        let connection = self.pool.get().await?;
        let content_types = None::<Vec<String>>;
        let (query, values) = build_find_args(
            "select c.* from collections as c ",
            "c",
            attributes,
            &content_types,
            None,
            &offset,
            &limit,
        );
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_trait_ids(&self, id: &Uuid) -> Result<Vec<String>, Error> {
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

    pub async fn get_all(&self, offset: i64, limit: i64) -> Result<Vec<Collection>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from collections order by name offset $1 limit $2")
            .await?;
        let rows = connection.query(&stmt, &[&offset, &limit]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get(&self, id: &Uuid) -> Result<Option<Collection>, Error> {
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

    #[allow(dead_code)]
    pub async fn get_txn(
        &self,
        txn: &Transaction<'_>,
        id: &Uuid,
    ) -> Result<Option<Collection>, Error> {
        let stmt = txn
            .prepare_cached("select * from collections where id = $1")
            .await?;
        let rows = txn.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    pub async fn get_parents(
        &self,
        id: &Uuid,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Collection>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select c.* from collections c inner join collection_items ci on (c.id = ci.collection_id) where ci.child_collection_id = $1 offset $2 limit $3")
            .await?;
        let rows = connection.query(&stmt, &[id, &offset, &limit]).await?;
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
            return Err(Error::new(
                "you must supply either a child collection id or child metadata id",
            ));
        }
        if child_collection_id.is_some() && child_metadata_id.is_some() {
            return Err(Error::new(
                "you can only supply either a child collection id or child metadata id",
            ));
        }
        let connection = self.pool.get().await?;
        if let Some(child_id) = child_collection_id {
            let stmt = connection.prepare_cached("update collection_items set attributes = $1 where collection_id = $2 and child_collection_id = $3").await?;
            connection
                .execute(&stmt, &[&attributes, collection_id, &child_id])
                .await?;
        } else if let Some(child_id) = child_metadata_id {
            let stmt = connection.prepare_cached("update collection_items set attributes = $1 where collection_id = $2 and child_metadata_id = $3").await?;
            connection
                .execute(&stmt, &[&attributes, collection_id, &child_id])
                .await?;
        }
        Ok(())
    }

    pub async fn set_public(&self, id: &Uuid, public: bool) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update collections set public = $1, modified = now() where id = $2")
            .await?;
        connection.execute(&stmt, &[&public, id]).await?;
        self.on_collection_changed(id).await?;
        Ok(())
    }

    pub async fn set_public_list(&self, id: &Uuid, public: bool) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update collections set public_list = $1, modified = now() where id = $2",
            )
            .await?;
        connection.execute(&stmt, &[&public, id]).await?;
        self.on_collection_changed(id).await?;
        Ok(())
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select collection_id from collection_items where child_collection_id = $1",
            )
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        let collection_ids: Vec<Uuid> = rows.iter().map(|r| r.get("collection_id")).collect();
        let stmt = connection
            .prepare_cached("delete from collections where id = $1")
            .await?;
        connection.execute(&stmt, &[id]).await?;
        self.on_collection_changed(id).await?;
        for collection_id in collection_ids {
            self.on_collection_changed(&collection_id).await?;
        }
        Ok(())
    }

    pub async fn get_children(
        &self,
        collection: &Collection,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<CollectionChild>, Error> {
        let mut values = Vec::new();
        let mut names = Vec::new();
        values.push(&collection.id as &(dyn ToSql + Sync));
        let ordering = if let Some(ordering) = &collection.ordering {
            build_ordering_names(ordering, &mut names);
            build_ordering("attributes", 2, ordering, &mut values, &names)
        } else {
            String::new()
        };
        let mut query = "select child_collection_id, child_metadata_id, collection_items.attributes from collection_items ".to_owned();
        if !ordering.is_empty() {
            query.push_str(" where collection_id = $1 ");
            query.push_str(ordering.as_str());
        } else {
            query.push_str(" left join collections on (child_collection_id = collections.id) ");
            query.push_str(" left join metadata on (child_metadata_id = metadata.id) ");
            query.push_str(" where collection_id = $1");
            query.push_str(" order by lower(collections.name) asc, lower(metadata.name) asc");
        }
        query.push_str(
            format!(" offset ${} limit ${}", values.len() + 1, values.len() + 2).as_str(),
        );
        values.push(&offset as &(dyn ToSql + Sync));
        values.push(&limit as &(dyn ToSql + Sync));
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_child_collections(
        &self,
        collection: &Collection,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Collection>, Error> {
        let mut values = Vec::new();
        let mut names = Vec::new();
        values.push(&collection.id as &(dyn ToSql + Sync));
        let ordering = if let Some(ordering) = &collection.ordering {
            build_ordering_names(ordering, &mut names);
            build_ordering("ci.attributes", 2, ordering, &mut values, &names)
        } else {
            String::new()
        };
        let mut query = "select c.*, ci.attributes as item_attributes from collections c inner join collection_items ci on (ci.child_collection_id = c.id and ci.collection_id = $1) ".to_owned();
        if ordering.is_empty() {
            query.push_str("order by name asc");
        } else {
            query.push_str(ordering.as_str());
        }
        query.push_str(
            format!(" offset ${} limit ${}", values.len() + 1, values.len() + 2).as_str(),
        );
        values.push(&offset as &(dyn ToSql + Sync));
        values.push(&limit as &(dyn ToSql + Sync));
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_child_metadata(
        &self,
        collection: &Collection,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Metadata>, Error> {
        let mut values = Vec::new();
        let mut names = Vec::new();
        values.push(&collection.id as &(dyn ToSql + Sync));
        let ordering = if let Some(ordering) = &collection.ordering {
            build_ordering_names(ordering, &mut names);
            build_ordering("ci.attributes", 2, ordering, &mut values, &names)
        } else {
            String::new()
        };
        let mut query = "select m.*, ci.attributes as item_attributes from metadata m inner join collection_items ci on (ci.child_metadata_id = m.id and ci.collection_id = $1) ".to_owned();
        if ordering.is_empty() {
            query.push_str("order by name asc");
        } else {
            query.push_str(ordering.as_str());
        }
        query.push_str(
            format!(" offset ${} limit ${}", values.len() + 1, values.len() + 2).as_str(),
        );
        values.push(&offset as &(dyn ToSql + Sync));
        values.push(&limit as &(dyn ToSql + Sync));
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn add(&self, collection: &CollectionInput) -> Result<Uuid, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let parent_id = if let Some(id) = &collection.parent_collection_id {
            Uuid::parse_str(id)?
        } else {
            Uuid::parse_str("00000000-0000-0000-0000-000000000000")?
        };
        match self.add_txn(&txn, collection).await {
            Ok(value) => {
                txn.commit().await?;
                self.on_collection_changed(&value).await?;
                self.on_collection_changed(&parent_id).await?;
                Ok(value)
            }
            Err(err) => {
                txn.rollback().await?;
                Err(err)
            }
        }
    }

    pub async fn edit(
        &self,
        id: &Uuid,
        collection: &CollectionInput,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;

        match self.edit_txn(&txn, id, collection).await {
            Ok(value) => {
                txn.commit().await?;
                self.on_collection_changed(id).await?;
                Ok(value)
            }
            Err(err) => {
                txn.rollback().await?;
                Err(err)
            }
        }
    }

    pub async fn add_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        collection: &CollectionInput,
    ) -> Result<Uuid, Error> {
        let stmt: Statement = if collection.collection_type.unwrap_or(CollectionType::Folder)
            == CollectionType::Root
        {
            txn.prepare("insert into collections (id, name, description, type, labels, attributes, ordering) values ('00000000-0000-0000-0000-000000000000', $1, $2, $3, $4, $5, $6) returning id").await?
        } else {
            txn.prepare("insert into collections (name, description, type, labels, attributes, ordering) values ($1, $2, $3, $4, $5, $6) returning id").await?
        };
        let labels = collection.labels.clone().unwrap_or_default();
        let ordering = collection
            .ordering
            .as_ref()
            .map(|ordering| serde_json::to_value(ordering).unwrap());
        let rows = txn
            .query(
                &stmt,
                &[
                    &collection.name,
                    &collection.description,
                    &collection.collection_type.unwrap_or(CollectionType::Folder),
                    &labels,
                    &collection.attributes.as_ref().or(Some(&Value::Null)),
                    &ordering,
                ],
            )
            .await?;

        let id = rows.first().unwrap().get(0);

        if let Some(trait_ids) = &collection.trait_ids {
            for trait_id in trait_ids {
                self.add_trait_txn(txn, &id, trait_id).await?
            }
        }

        Ok(id)
    }

    #[allow(dead_code)]
    async fn delete_trait_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare("delete from collection_traits where collection_id = $1")
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
            .prepare("insert into collection_traits (collection_id, trait_id) values ($1, $2)")
            .await?;
        txn.execute(&stmt, &[id, trait_id]).await?;
        Ok(())
    }

    async fn edit_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
        collection: &CollectionInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare("update collections set name = $1, description = $2, type = $3, labels = $4, attributes = $5, ordering = $6 where id = $7").await?;
        let labels = collection.labels.clone().unwrap_or_default();
        let ordering = collection
            .ordering
            .as_ref()
            .map(|ordering| serde_json::to_value(ordering).unwrap());
        txn.execute(
            &stmt,
            &[
                &collection.name,
                &collection.description,
                &collection.collection_type.unwrap_or(CollectionType::Folder),
                &labels,
                &collection.attributes.as_ref().or(Some(&Value::Null)),
                &ordering,
                id,
            ],
        )
        .await?;

        self.delete_trait_txn(txn, id).await?;
        if let Some(trait_ids) = &collection.trait_ids {
            for trait_id in trait_ids {
                self.add_trait_txn(txn, id, trait_id).await?
            }
        }

        Ok(())
    }

    pub async fn add_child_collection(
        &self,
        id: &Uuid,
        collection_id: &Uuid,
        attributes: &Option<Value>,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "insert into collection_items (collection_id, child_collection_id, attributes) values ($1, $2, $3)",
            )
            .await?;
        connection
            .execute(&stmt, &[id, collection_id, attributes])
            .await?;
        self.on_collection_changed(id).await?;
        self.on_collection_changed(collection_id).await?;
        Ok(())
    }

    pub async fn add_child_collection_txn(
        &self,
        txn: &Transaction<'_>,
        id: &Uuid,
        collection_id: &Uuid,
        attributes: &Option<Value>,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into collection_items (collection_id, child_collection_id, attributes) values ($1, $2, $3)").await?;
        txn.execute(&stmt, &[id, collection_id, attributes]).await?;
        Ok(())
    }

    pub async fn add_child_metadata(
        &self,
        id: &Uuid,
        metadata_id: &Uuid,
        attributes: &Option<Value>,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "insert into collection_items (collection_id, child_metadata_id, attributes) values ($1, $2, $3)",
            )
            .await?;
        connection
            .execute(&stmt, &[id, metadata_id, attributes])
            .await?;
        self.on_collection_changed(id).await?;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    pub async fn add_child_metadata_txn(
        &self,
        txn: &Transaction<'_>,
        id: &Uuid,
        metadata_id: &Uuid,
        attributes: &Option<Value>,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare_cached(
                "insert into collection_items (collection_id, child_metadata_id, attributes) values ($1, $2, $3)",
            )
            .await?;
        txn.execute(&stmt, &[id, metadata_id, attributes]).await?;
        self.on_collection_changed(id).await?;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    pub async fn remove_child_collection(
        &self,
        id: &Uuid,
        collection_id: &Uuid,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "delete from collection_items where collection_id = $1 and child_collection_id = $2",
            )
            .await?;
        connection.execute(&stmt, &[id, collection_id]).await?;
        self.on_collection_changed(collection_id).await?;
        self.on_collection_changed(id).await?;
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
        self.on_collection_changed(id).await?;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    pub async fn set_attributes(
        &self,
        collection_id: &Uuid,
        attributes: Value,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update collections set attributes = $1, modified = now() where id = $2",
            )
            .await?;
        connection
            .execute(&stmt, &[&attributes, &collection_id])
            .await?;
        self.on_collection_changed(collection_id).await?;
        Ok(())
    }

    pub async fn set_ordering(
        &self,
        collection_id: &Uuid,
        ordering: Value,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update collections set ordering = $1, modified = now() where id = $2")
            .await?;
        connection
            .execute(&stmt, &[&ordering, &collection_id])
            .await?;
        self.on_collection_changed(collection_id).await?;
        Ok(())
    }

    pub async fn add_all(
        &self,
        ctx: &BoscaContext,
        collections: &mut [CollectionChildInput],
    ) -> Result<Vec<Uuid>, Error> {
        let mut search_documents = Vec::new();
        let ids = {
            let mut conn = self.pool.get().await?;
            let txn = conn.transaction().await?;
            let ids = self
                .add_all_txn(ctx, &txn, collections, &mut search_documents, false, None)
                .await?;
            txn.commit().await?;
            ids
        };
        let new_ctx = ctx.clone();
        tokio::spawn(async move {
            match new_ctx.workflow.get_default_search_storage_system().await {
                Ok(Some(storage_system)) => {
                    if let Err(err) =
                        index_documents(&new_ctx, &search_documents, &storage_system).await
                    {
                        error!("failed to index documents: {}", err.message);
                    }
                }
                Ok(None) => {
                    error!("failed to index documents, missing storage system");
                }
                Err(err) => {
                    error!(
                        "failed to get storage system to index documents: {}",
                        err.message
                    );
                }
            }
        });
        for id in &ids {
            self.on_collection_changed(&id.0).await?;
            self.on_collection_changed(&id.1).await?;
        }
        Ok(ids.into_iter().map(|(id, _)| id).collect())
    }

    #[allow(clippy::too_many_arguments)]
    async fn add_all_txn(
        &self,
        ctx: &BoscaContext,
        txn: &Transaction<'_>,
        collections: &mut [CollectionChildInput],
        search_documents: &mut Vec<SearchDocumentInput>,
        ignore_permission_check: bool,
        permissions: Option<Vec<Permission>>,
    ) -> Result<Vec<(Uuid, Uuid)>, Error> {
        let mut new_collections = Vec::new();
        for collection_child in collections.iter_mut() {
            let collection = &mut collection_child.collection;
            let has_collection_id = collection.parent_collection_id.is_some();
            let parent_collection_id = match &collection.parent_collection_id {
                Some(id) => Uuid::parse_str(id.as_str())?,
                None => Uuid::parse_str("00000000-0000-0000-0000-000000000000")?,
            };
            if !ignore_permission_check {
                ctx.check_collection_action_txn(txn, &parent_collection_id, PermissionAction::Edit)
                    .await?;
            }
            let id = self.add_txn(txn, collection).await?;
            let permissions = if let Some(permissions) = &permissions {
                permissions.clone()
            } else {
                ctx.content
                    .collection_permissions
                    .get_txn(txn, &parent_collection_id)
                    .await?
            };
            for permission in permissions.iter() {
                let collection_permission = Permission {
                    entity_id: id,
                    group_id: permission.group_id,
                    action: permission.action,
                };
                ctx.content
                    .collection_permissions
                    .add_txn(txn, &collection_permission)
                    .await?
            }
            if has_collection_id {
                self.add_child_collection_txn(
                    txn,
                    &parent_collection_id,
                    &id,
                    &collection_child.attributes,
                )
                .await?;
            }
            if let Some(children) = &mut collection.collections {
                for child in children.iter_mut() {
                    child.collection.parent_collection_id = Some(id.to_string());
                }
                Box::pin(self.add_all_txn(
                    ctx,
                    txn,
                    children,
                    search_documents,
                    true,
                    Some(permissions.clone()),
                ))
                .await?;
            }
            if let Some(children) = &mut collection.metadata {
                for child in children.iter_mut() {
                    child.metadata.parent_collection_id = Some(id.to_string());
                }
                ctx.content.metadata.add_all_txn(
                    ctx,
                    txn,
                    children,
                    search_documents,
                    true,
                    Some(permissions.clone()),
                )
                .await?;
            }
            if collection.index.unwrap_or(true) {
                search_documents.push(SearchDocumentInput {
                    collection_id: Some(id.to_string()),
                    metadata_id: None,
                    profile_id: None,
                    content: "".to_owned(),
                });
            }
            new_collections.push((id, parent_collection_id));
        }
        Ok(new_collections)
    }

}
