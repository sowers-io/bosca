use crate::context::BoscaContext;
use crate::models::content::guide_history::GuideHistory;
use crate::models::content::guide_progress::GuideProgress;
use crate::models::profiles::profile::{Profile, ProfileInput};
use crate::models::profiles::profile_attribute::ProfileAttribute;
use crate::models::profiles::profile_attribute_type::{
    ProfileAttributeType, ProfileAttributeTypeInput,
};
use crate::models::profiles::profile_bookmark::ProfileBookmark;
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::workflow::core_workflow_ids::PROFILE_UPDATE_STORAGE;
use async_graphql::Error;
use bosca_database::TracingPool;
use deadpool_postgres::{GenericClient, Transaction};
use serde_json::Value;
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone)]
pub struct ProfileDataStore {
    pool: TracingPool,
}

impl Debug for ProfileDataStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProfileDataStore").finish()
    }
}

impl ProfileDataStore {
    pub fn new(pool: TracingPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(skip(self, offset, limit))]
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

    #[tracing::instrument(skip(self, id))]
    pub async fn get_by_id(&self, id: &Uuid) -> async_graphql::Result<Option<Profile>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profiles where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, id))]
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

    #[tracing::instrument(skip(self, id))]
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

    #[tracing::instrument(skip(self))]
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

    #[tracing::instrument(skip(self, attribute))]
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

    #[tracing::instrument(skip(self, attribute))]
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

    #[tracing::instrument(skip(self, id))]
    pub async fn delete_profile_attribute_type(&self, id: &str) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from profile_attribute_types where id = $1")
            .await?;
        let id = id.to_string();
        connection.execute(&stmt, &[&id]).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, principal, profile, collection_id))]
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
            return Err(Error::new("failed to create profile"));
        }
        let id: Uuid = results[0].get("id");
        let stmt = txn.prepare_cached("insert into slugs (slug, profile_id) values (case when length($1) > 0 then $1 else slugify($2) end, $3) on conflict (slug) do update set slug = slugify($2) || nextval('duplicate_slug_seq')").await?;
        txn.execute(&stmt, &[&profile.slug, &profile.name, &id])
            .await?;
        let stmt = txn.prepare_cached("insert into profile_attributes (profile, type_id, visibility, confidence, priority, source, attributes, metadata_id) values ($1, $2, $3, $4, $5, $6, $7, $8)").await?;
        for attribute in profile.attributes.iter() {
            let metadata_id = attribute
                .metadata_id
                .as_ref()
                .map(|id| Uuid::parse_str(id).unwrap());
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
        if collection_id.is_some() {
            self.update_storage(ctx, &id).await?;
        }
        Ok(id)
    }

    #[tracing::instrument(skip(self, txn,  principal, profile, collection_id))]
    pub async fn add_txn(
        &self,
        txn: &Transaction<'_>,
        principal: Option<Uuid>,
        profile: &ProfileInput,
        collection_id: Option<Uuid>,
    ) -> Result<Uuid, Error> {
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
            return Err(Error::new("failed to create profile"));
        }
        let id: Uuid = results[0].get("id");
        let stmt = txn.prepare_cached("insert into slugs (slug, profile_id) values (case when length($1) > 0 then $1 else slugify($2) end, $3) on conflict (slug) do update set slug = slugify($2) || nextval('duplicate_slug_seq')").await?;
        txn.execute(&stmt, &[&profile.slug, &profile.name, &id])
            .await?;
        let stmt = txn.prepare_cached("insert into profile_attributes (profile, type_id, visibility, confidence, priority, source, attributes, metadata_id) values ($1, $2, $3, $4, $5, $6, $7, $8)").await?;
        for attribute in profile.attributes.iter() {
            let metadata_id = attribute
                .metadata_id
                .as_ref()
                .map(|id| Uuid::parse_str(id).unwrap());
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
        Ok(id)
    }

    #[tracing::instrument(skip(self, profile_id))]
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

    #[tracing::instrument(skip(self, profile_id))]
    pub async fn get_progressions(
        &self,
        profile_id: &Uuid,
        offset: i64,
        limit: i64,
    ) -> async_graphql::Result<Vec<GuideProgress>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profile_guide_progress where profile_id = $1 order by modified desc offset $2 limit $3")
            .await?;
        let rows = connection
            .query(&stmt, &[profile_id, &offset, &limit])
            .await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, profile_id))]
    pub async fn get_progress(
        &self,
        profile_id: &Uuid,
        metadata_id: &Uuid,
        version: i32,
    ) -> async_graphql::Result<Option<GuideProgress>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profile_guide_progress where profile_id = $1 and metadata_id = $2 and version = $3")
            .await?;
        let rows = connection
            .query(&stmt, &[profile_id, metadata_id, &version])
            .await?;
        Ok(rows.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, profile_id))]
    pub async fn get_progression_count(
        &self,
        profile_id: &Uuid,
    ) -> async_graphql::Result<i64, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select count(*) as c from profile_guide_progress where profile_id = $1",
            )
            .await?;
        let rows = connection.query(&stmt, &[profile_id]).await?;
        let r = rows.first().unwrap();
        Ok(r.get("c"))
    }

    #[tracing::instrument(skip(self, profile_id, offset, limit))]
    pub async fn get_histories(
        &self,
        profile_id: &Uuid,
        offset: i64,
        limit: i64,
    ) -> async_graphql::Result<Vec<GuideHistory>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profile_guide_history where profile_id = $1 order by completed desc offset $2 limit $3")
            .await?;
        let rows = connection
            .query(&stmt, &[profile_id, &offset, &limit])
            .await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, profile_id))]
    pub async fn get_histories_count(
        &self,
        profile_id: &Uuid,
    ) -> async_graphql::Result<i64, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select count(*) as c from profile_guide_history where profile_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[profile_id]).await?;
        let r = rows.first().unwrap();
        Ok(r.get("c"))
    }

    #[tracing::instrument(skip(self, profile_id, metadata_id, version))]
    pub async fn get_history(
        &self,
        profile_id: &Uuid,
        metadata_id: &Uuid,
        version: i32,
        offset: i64,
        limit: i64,
    ) -> async_graphql::Result<Vec<GuideHistory>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profile_guide_history where profile_id = $1 and metadata_id = $2 and version = $3 order by completed desc offset $4 limit $5")
            .await?;
        let rows = connection
            .query(&stmt, &[profile_id, metadata_id, &version, &offset, &limit])
            .await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, profile_id))]
    pub async fn get_history_count(
        &self,
        profile_id: &Uuid,
        metadata_id: &Uuid,
        version: i32,
    ) -> async_graphql::Result<i64, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select count(*) as c from profile_guide_history where profile_id = $1 and metadata_id = $2 and version = $3")
            .await?;
        let rows = connection
            .query(&stmt, &[profile_id, metadata_id, &version])
            .await?;
        let r = rows.first().unwrap();
        Ok(r.get("c"))
    }

    #[tracing::instrument(skip(self, ctx, principal, profile))]
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
            return Err(Error::new("failed to find profile by principal"));
        }
        let id: Uuid = results[0].get("id");
        self.edit_profile_attributes(&txn, &id, profile).await?;
        txn.commit().await?;
        self.update_storage(ctx, &id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, profile))]
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
                "update profiles set name = $1, visibility = $2 where id = $3 returning id",
            )
            .await?;
        let results = txn
            .query(&stmt, &[&profile.name, &profile.visibility, &id])
            .await?;
        if results.is_empty() {
            return Err(Error::new("failed find profile by id"));
        }
        let id: Uuid = results[0].get("id");
        self.edit_profile_attributes(&txn, &id, profile).await?;
        txn.commit().await?;
        self.update_storage(ctx, &id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, profile_id, attribute_id))]
    pub async fn delete_profile_attribute(
        &self,
        profile_id: &Uuid,
        attribute_id: &Uuid,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from profile_attributes where profile = $1 and id = $2")
            .await?;
        connection
            .execute(&stmt, &[profile_id, attribute_id])
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, profile_id, profile))]
    async fn edit_profile_attributes(
        &self,
        txn: &Transaction<'_>,
        profile_id: &Uuid,
        profile: &ProfileInput,
    ) -> Result<(), Error> {
        if let Some(slug) = &profile.slug {
            let stmt = txn
                .prepare_cached("update slugs set slug = $1 where profile_id = $2")
                .await?;
            txn.execute(&stmt, &[slug, profile_id]).await?;
        }
        let update_stmt = txn.prepare("update profile_attributes set visibility = $1, confidence = $2, priority = $3, source = $4, attributes = $5, metadata_id = $6 where id = $7 and profile = $8").await?;
        let insert_stmt = txn.prepare_cached("insert into profile_attributes (profile, type_id, visibility, confidence, priority, source, attributes, metadata_id) values ($1, $2, $3, $4, $5, $6, $7, $8)").await?;
        for attribute in profile.attributes.iter() {
            let metadata_id = attribute
                .metadata_id
                .as_ref()
                .map(|id| Uuid::parse_str(id).unwrap());
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
                        profile_id,
                    ],
                )
                .await?;
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
                )
                .await?;
            }
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id))]
    pub async fn update_storage(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        let mut request = EnqueueRequest {
            workflow_id: Some(PROFILE_UPDATE_STORAGE.to_string()),
            profile_id: Some(*id),
            ..Default::default()
        };
        ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, profile_id))]
    pub async fn get_bookmarks_count(
        &self,
        profile_id: &Uuid,
    ) -> async_graphql::Result<i64, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select count(*) as c from profile_bookmarks where profile_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[profile_id]).await?;
        let row = rows.first().unwrap();
        let c = row.get("c");
        Ok(c)
    }

    #[tracing::instrument(skip(self, profile_id))]
    pub async fn get_bookmarks(
        &self,
        profile_id: &Uuid,
    ) -> async_graphql::Result<Vec<ProfileBookmark>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profile_bookmarks where profile_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[profile_id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, profile_id))]
    pub async fn get_bookmark(
        &self,
        profile_id: &Uuid,
        metadata_id: Option<Uuid>,
        metadata_version: Option<i32>,
        collection_id: Option<Uuid>,
    ) -> async_graphql::Result<Option<ProfileBookmark>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from profile_bookmarks where profile_id = $1 and ((metadata_id = $2 and metadata_version = $3) or (collection_id = $4))",
            ).await?;
        let rows = connection
            .query(
                &stmt,
                &[&profile_id, &metadata_id, &metadata_version, &collection_id],
            )
            .await?;
        Ok(rows.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, profile_id, metadata_id, metadata_version, collection_id))]
    pub async fn add_bookmark(
        &self,
        _: &BoscaContext,
        profile_id: &Uuid,
        metadata_id: Option<Uuid>,
        metadata_version: Option<i32>,
        collection_id: Option<Uuid>,
    ) -> async_graphql::Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        if metadata_id.is_some() && metadata_version.is_some() {
            let stmt = txn
                .prepare_cached(
                    "insert into profile_bookmarks (profile_id, metadata_id, metadata_version) values ($1, $2, $3) on conflict (profile_id, metadata_id, metadata_version, collection_id) do nothing",
                )
                .await?;
            txn.execute(&stmt, &[&profile_id, &metadata_id, &metadata_version])
                .await?;
        } else {
            let stmt = txn
                .prepare_cached(
                    "insert into profile_bookmarks (profile_id, collection_id) values ($1, $2) on conflict (profile_id, metadata_id, metadata_version, collection_id) do nothing",
                ).await?;
            txn.execute(&stmt, &[&profile_id, &collection_id]).await?;
        }
        txn.commit().await?;
        // TODO: fire workflow
        Ok(())
    }

    #[tracing::instrument(skip(self, profile_id, metadata_id, metadata_version, collection_id))]
    pub async fn delete_bookmark(
        &self,
        _: &BoscaContext,
        profile_id: &Uuid,
        metadata_id: Option<Uuid>,
        metadata_version: Option<i32>,
        collection_id: Option<Uuid>,
    ) -> async_graphql::Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        if metadata_id.is_some() && metadata_version.is_some() {
            let stmt = txn
                .prepare_cached(
                    "delete from profile_bookmarks where profile_id = $1 and metadata_id = $2 and metadata_version = $3",
                )
                .await?;
            txn.execute(&stmt, &[&profile_id, &metadata_id, &metadata_version])
                .await?;
        } else {
            let stmt = txn
                .prepare_cached(
                    "delete from profile_bookmarks where profile_id = $1 and collection_id = $2",
                )
                .await?;
            txn.execute(&stmt, &[&profile_id, &collection_id]).await?;
        }
        txn.commit().await?;
        // TODO: fire workflow
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, profile_id, metadata_id, metadata_version, attributes))]
    pub async fn add_progress(
        &self,
        ctx: &BoscaContext,
        profile_id: &Uuid,
        metadata_id: &Uuid,
        metadata_version: i32,
        attributes: &Value,
        step_id: i64,
    ) -> async_graphql::Result<bool, Error> {
        let steps = ctx
            .content
            .guides
            .get_guide_step_ids(metadata_id, metadata_version)
            .await?;
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let (completed, attributes) = if steps.contains(&step_id) {
            let stmt = txn
                .prepare_cached("insert into profile_guide_progress as p (profile_id, metadata_id, version, attributes, completed_step_ids) values ($1, $2, $3, $4, ARRAY[$5]::bigint[]) on conflict (profile_id, metadata_id, version) do update set modified = now(), attributes = coalesce(p.attributes, '{}'::jsonb) || $4, completed_step_ids = array_append(p.completed_step_ids, $5) where not (p.completed_step_ids @> ARRAY[$5]::bigint[]) returning array_length(p.completed_step_ids, 1) as l, attributes")
                .await?;
            let results = txn
                .query(
                    &stmt,
                    &[
                        &profile_id,
                        &metadata_id,
                        &metadata_version,
                        &attributes,
                        &step_id,
                    ],
                )
                .await?;
            if results.is_empty() {
                return Ok(false);
            }
            let result = results.first().unwrap();
            let completed: Option<i32> = result.get("l");
            let attributes: Option<Value> = result.get("attributes");
            (completed, attributes)
        } else {
            let stmt = txn
                .prepare_cached("insert into profile_guide_progress as p (profile_id, metadata_id, version, attributes, completed_step_ids) values ($1, $2, $3, $4, '{}'::bigint[]) on conflict (profile_id, metadata_id, version) do update set modified = now(), attributes = coalesce(p.attributes, '{}'::jsonb) || $4 returning array_length(p.completed_step_ids, 1) as l, attributes")
                .await?;
            let results = txn
                .query(
                    &stmt,
                    &[&profile_id, &metadata_id, &metadata_version, &attributes],
                )
                .await?;
            if results.is_empty() {
                return Ok(false);
            }
            let result = results.first().unwrap();
            let completed: Option<i32> = result.get("l");
            let attributes: Option<Value> = result.get("attributes");
            (completed, attributes)
        };
        if completed.unwrap_or(0) == steps.len() as i32 {
            let stmt = txn
                .prepare_cached("insert into profile_guide_history (profile_id, metadata_id, version, attributes, completed) values ($1, $2, $3, $4, now())")
                .await?;
            txn.execute(
                &stmt,
                &[&profile_id, &metadata_id, &metadata_version, &attributes],
            )
            .await?;
            let stmt = txn.prepare_cached("delete from profile_guide_progress where profile_id = $1 and metadata_id = $2 and version = $3")
                .await?;
            txn.execute(&stmt, &[&profile_id, &metadata_id, &metadata_version])
                .await?;
        }
        txn.commit().await?;
        // TODO: fire workflow
        Ok(true)
    }
}
