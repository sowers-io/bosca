use crate::models::profiles::profile::{Profile, ProfileInput};
use crate::models::profiles::profile_attribute::ProfileAttribute;
use async_graphql::Error;
use deadpool_postgres::{GenericClient, Pool};
use std::sync::Arc;
use uuid::Uuid;
use crate::models::profiles::profile_attribute_type::ProfileAttributeType;

#[derive(Clone)]
pub struct ProfileDataStore {
    pool: Arc<Pool>,
}

impl ProfileDataStore {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    pub async fn get_by_id(
        &self,
        id: &Uuid,
    ) -> async_graphql::Result<Option<Profile>, Error> {
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
            return Ok(None)
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

    pub async fn add(
        &self,
        principal: &Uuid,
        profile: &ProfileInput,
        collection_id: &Uuid,
    ) -> async_graphql::Result<Uuid, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into profiles (principal, name, visibility, collection_id) values ($1, $2, $3, $4) returning id").await?;
        let results = txn
            .query(&stmt, &[&principal, &profile.name, &profile.visibility, &collection_id])
            .await?;
        if results.is_empty() {
            return Err(Error::new("failed to create principal"));
        }
        let id = results[0].get("id");
        let stmt = txn.prepare_cached("insert into profile_attributes (profile, type_id, visibility, confidence, priority, source, attributes) values ($1, $2, $3, $4, $5, $6, $7)").await?;
        for attribute in profile.attributes.iter() {
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
                ],
            )
            .await?;
        }
        txn.commit().await?;
        Ok(id)
    }

    pub async fn get_attributes(
        &self,
        profile_id: &Uuid,
    ) -> async_graphql::Result<Vec<ProfileAttribute>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profile_attributes where profiles = $1 and (expired is null or expired > now()) order by priority asc, confidence desc")
            .await?;
        let rows = connection.query(&stmt, &[profile_id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn edit(
        &self,
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
        let id: i64 = results[0].get("id");
        let update_stmt = txn.prepare_cached("update profile_attributes set visibility = $1, confidence = $2, priority = $3, source = $4, attributes = $5 where id = $6").await?;
        let insert_stmt = txn.prepare_cached("insert into profile_attributes (profile, type_id, visibility, confidence, priority, source, attributes) values ($1, $2, $3, $4, $5, $6, $7)").await?;
        for attribute in profile.attributes.iter() {
            if let Some(id) = attribute.id.as_ref() {
                let id = Uuid::parse_str(id)?;
                txn.execute(
                    &update_stmt,
                    &[
                        &attribute.visibility,
                        &attribute.confidence,
                        &attribute.priority,
                        &attribute.source,
                        &attribute.attributes,
                        &id,
                    ],
                )
                .await?;
            } else {
                txn.execute(
                    &insert_stmt,
                    &[
                        &id,
                        &attribute.type_id,
                        &attribute.visibility,
                        &attribute.confidence,
                        &attribute.priority,
                        &attribute.source,
                        &attribute.attributes,
                    ],
                )
                .await?;
            }
        }
        txn.commit().await?;
        Ok(())
    }
}
