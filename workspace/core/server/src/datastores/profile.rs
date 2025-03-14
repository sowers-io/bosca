use crate::context::BoscaContext;
use crate::models::profiles::profile::{Profile, ProfileInput};
use crate::models::profiles::profile_attribute::ProfileAttribute;
use crate::models::profiles::profile_attribute_type::{
    ProfileAttributeType, ProfileAttributeTypeInput,
};
use async_graphql::Error;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use std::sync::Arc;
use uuid::Uuid;
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::workflow::core_workflow_ids::PROFILE_INDEX;

#[derive(Clone)]
pub struct ProfileDataStore {
    pool: Arc<Pool>,
}

impl ProfileDataStore {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(
        &self,
        offset: i64,
        limit: i64,
    ) -> async_graphql::Result<Vec<Profile>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profiles order by id offset $1 limit $2")
            .await?;
        let rows = connection.query(&stmt, &[&offset, &limit]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_by_id(&self, id: &Uuid) -> async_graphql::Result<Option<Profile>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profiles where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    pub async fn get_by_principal(
        &self,
        id: &Uuid,
    ) -> async_graphql::Result<Option<Profile>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profiles where principal = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    pub async fn get_slug(&self, id: &Uuid) -> Result<Option<String>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select slug from slugs where profile_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().get("slug")))
    }

    pub async fn get_attribute_types(
        &self,
    ) -> async_graphql::Result<Vec<ProfileAttributeType>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profile_attribute_types")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn add_profile_attribute_type(
        &self,
        attribute: &ProfileAttributeTypeInput,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("insert into profile_attribute_types (id, name, description, visibility) values ($1, $2, $3, $4)")
            .await?;
        connection
            .execute(
                &stmt,
                &[
                    &attribute.id,
                    &attribute.name,
                    &attribute.description,
                    &attribute.visibility,
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn edit_profile_attribute_type(
        &self,
        attribute: &ProfileAttributeTypeInput,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update profile_attribute_types set name = $1, description = $2, visibility = $3 where id = $4")
            .await?;
        connection
            .execute(
                &stmt,
                &[
                    &attribute.name,
                    &attribute.description,
                    &attribute.visibility,
                    &attribute.id,
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn delete_profile_attribute_type(&self, id: &str) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from profile_attribute_types where id = $1")
            .await?;
        let id = id.to_string();
        connection.execute(&stmt, &[&id]).await?;
        Ok(())
    }

    pub async fn add(
        &self,
        ctx: &BoscaContext,
        principal: Option<Uuid>,
        profile: &ProfileInput,
        collection_id: Option<Uuid>,
    ) -> Result<Uuid, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into profiles (principal, name, visibility, collection_id) values ($1, $2, $3, $4) returning id").await?;
        let results = txn
            .query(
                &stmt,
                &[
                    &principal,
                    &profile.name,
                    &profile.visibility,
                    &collection_id,
                ],
            )
            .await?;
        if results.is_empty() {
            return Err(Error::new("failed to create principal"));
        }
        let id: Uuid = results[0].get("id");
        let stmt = txn.prepare_cached("insert into slugs (slug, profile_id) values (case when length($1) > 0 then $1 else slugify($2) end, $3) on conflict (slug) do update set slug = slugify($2) || nextval('duplicate_slug_seq')").await?;
        txn.execute(&stmt, &[&profile.slug, &profile.name, &id]).await?;
        let stmt = txn.prepare_cached("insert into profile_attributes (profile, type_id, visibility, confidence, priority, source, attributes, metadata_id) values ($1, $2, $3, $4, $5, $6, $7, $8)").await?;
        for attribute in profile.attributes.iter() {
            let metadata_id = attribute.metadata_id.as_ref().map(|id| Uuid::parse_str(id).unwrap());
            txn.execute(
                &stmt,
                &[
                    &id,
                    &attribute.type_id,
                    &attribute.visibility,
                    &attribute.confidence,
                    &attribute.priority,
                    &attribute.source,
                    &attribute.attributes,
                    &metadata_id,
                ],
            )
            .await?;
        }
        txn.commit().await?;
        self.index_profile(ctx, &id).await?;
        Ok(id)
    }

    pub async fn get_attributes(
        &self,
        profile_id: &Uuid,
    ) -> async_graphql::Result<Vec<ProfileAttribute>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profile_attributes where profile = $1 and (expiration is null or expiration > now()) order by priority asc, confidence desc")
            .await?;
        let rows = connection.query(&stmt, &[profile_id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn edit_by_principal(
        &self,
        ctx: &BoscaContext,
        principal: &Uuid,
        profile: &ProfileInput,
    ) -> async_graphql::Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "update profiles set name = $1, visibility = $2 where principal = $3 returning id",
            )
            .await?;
        let results = txn
            .query(&stmt, &[&profile.name, &profile.visibility, &principal])
            .await?;
        if results.is_empty() {
            return Err(Error::new("failed to create principal"));
        }
        let id: Uuid = results[0].get("id");
        self.edit_profile_attributes(&txn, &id, profile).await?;
        txn.commit().await?;
        self.index_profile(ctx, &id).await?;
        Ok(())
    }

    pub async fn edit_by_id(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        profile: &ProfileInput,
    ) -> async_graphql::Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "update profiles set name = $1, visibility = $2 where id = $3",
            )
            .await?;
        let results = txn
            .query(&stmt, &[&profile.name, &profile.visibility, &id])
            .await?;
        if results.is_empty() {
            return Err(Error::new("failed to create principal"));
        }
        let id: Uuid = results[0].get("id");
        self.edit_profile_attributes(&txn, &id, profile).await?;
        txn.commit().await?;
        self.index_profile(ctx, &id).await?;
        Ok(())
    }

    async fn edit_profile_attributes(&self, txn: &Transaction<'_>, profile_id: &Uuid, profile: &ProfileInput) -> Result<(), Error> {
        if let Some(slug) = &profile.slug {
            let stmt = txn.prepare_cached("update slugs set slug = $1 where profile_id = $2").await?;
            txn.execute(&stmt, &[slug, profile_id]).await?;
        }
        let update_stmt = txn.prepare("update profile_attributes set visibility = $1, confidence = $2, priority = $3, source = $4, attributes = $5, metadata_id = $6 where id = $7 and profile = $8").await?;
        let insert_stmt = txn.prepare_cached("insert into profile_attributes (profile, type_id, visibility, confidence, priority, source, attributes, metadata_id) values ($1, $2, $3, $4, $5, $6, $7, $8)").await?;
        for attribute in profile.attributes.iter() {
            let metadata_id = attribute.metadata_id.as_ref().map(|id| Uuid::parse_str(id).unwrap());
            if let Some(id) = attribute.id.as_ref() {
                let attr_id = Uuid::parse_str(id)?;
                txn.execute(
                    &update_stmt,
                    &[
                        &attribute.visibility,
                        &attribute.confidence,
                        &attribute.priority,
                        &attribute.source,
                        &attribute.attributes,
                        &metadata_id,
                        &attr_id,
                        profile_id
                    ],
                ).await?;
            } else {
                txn.execute(
                    &insert_stmt,
                    &[
                        profile_id,
                        &attribute.type_id,
                        &attribute.visibility,
                        &attribute.confidence,
                        &attribute.priority,
                        &attribute.source,
                        &attribute.attributes,
                        &metadata_id,
                    ],
                ).await?;
            }
        }
        Ok(())
    }

    pub async fn index_profile(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
    ) -> Result<(), Error> {
        let mut request = EnqueueRequest {
            workflow_id: Some(PROFILE_INDEX.to_string()),
            profile_id: Some(*id),
            ..Default::default()
        };
        ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
        Ok(())
    }
}
