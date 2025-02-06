use crate::models::profile::profile::{Profile, ProfileInput};
use async_graphql::Error;
use deadpool_postgres::{GenericClient, Pool};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct ProfileDataStore {
    pool: Arc<Pool>,
}

impl ProfileDataStore {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    pub async fn get_profile_by_principal(
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

    pub async fn add_profile(
        &self,
        principal: &Uuid,
        profile: &ProfileInput,
    ) -> async_graphql::Result<Uuid, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into profiles (principal, name, visibility) values ($1, $2, $3) returning id").await?;
        let results = txn
            .query(&stmt, &[&principal, &profile.name, &profile.visibility])
            .await?;
        if results.is_empty() {
            return Err(Error::new("failed to create principal"));
        }
        let id = results[0].get("id");
        let stmt = txn.prepare_cached("insert into profile_attributes (profile, type_id, visibility, confidence, priority, source, attributes) values ($1, $2, $3, $4, $5, $6, $7)").await?;
        for attribute in profile.attributes.iter() {
            let type_id = Uuid::parse_str(&attribute.type_id)?;
            txn.execute(
                &stmt,
                &[
                    &id,
                    &type_id,
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

    pub async fn edit_profile(
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
                let type_id = Uuid::parse_str(&attribute.type_id)?;
                txn.execute(
                    &insert_stmt,
                    &[
                        &id,
                        &type_id,
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
