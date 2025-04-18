use std::fmt::Debug;
use crate::datastores::cache::manager::BoscaCacheManager;
use crate::datastores::security_cache::SecurityCache;
use crate::models::security::credentials::{Credential, CredentialType, PasswordCredential};
use crate::models::security::group::Group;
use crate::models::security::group_type::GroupType;
use crate::models::security::password::verify;
use crate::models::security::principal::Principal;
use crate::security::jwt::Jwt;
use crate::security::token::Token;
use crate::util::signed_url::{sign_url, verify_signed_url};
use async_graphql::*;
use chrono::{DateTime, Utc};
use deadpool_postgres::{GenericClient, Object};
use log::warn;
use serde_json::Value;
use uuid::Uuid;
use bosca_database::TracingPool;

#[derive(Clone)]
pub struct SecurityDataStore {
    cache: SecurityCache,
    pool: TracingPool,
    jwt: Jwt,
    url_secret_key: String,
}

impl Debug for SecurityDataStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecurityDataStore").finish()
    }
}

pub const ADMINISTRATORS_GROUP: &str = "administrators";
pub const SERVICE_ACCOUNT_GROUP: &str = "sa";
pub const MODEL_MANAGERS_GROUP: &str = "model.managers";
pub const WORKFLOW_MANAGERS_GROUP: &str = "workflow.managers";

pub struct RefreshToken {
    pub token: String
}

impl Debug for RefreshToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RefreshToken").finish()
    }
}

impl SecurityDataStore {
    pub async fn new(
        cache: &mut BoscaCacheManager,
        pool: TracingPool,
        jwt: Jwt,
        url_secret_key: String,
    ) -> Self {
        Self {
            cache: SecurityCache::new(cache).await,
            pool,
            jwt,
            url_secret_key,
        }
    }

    pub fn sign_url(&self, url: &str) -> String {
        sign_url(url, &self.url_secret_key, 3600)
    }

    pub fn verify_signed_url(&self, url: &str) -> bool {
        verify_signed_url(url, &self.url_secret_key)
    }

    #[allow(dead_code)]
    pub async fn add_group(
        &self,
        name: &String,
        description: &String,
        group_type: GroupType,
    ) -> Result<Group, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("insert into groups (name, description, type) values ($1, $2, $3::group_type) returning id")
            .await?;
        let results = connection
            .query(&stmt, &[name, description, &group_type])
            .await?;
        let id: Uuid = results.first().unwrap().get(0);
        let group = Group::new(id, name.clone(), group_type);
        self.cache.cache_group(&group).await;
        Ok(group)
    }

    #[tracing::instrument(skip(self, offset, limit))]
    pub async fn get_groups(&self, offset: i64, limit: i64) -> Result<Vec<Group>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from groups where type = 'system'::group_type order by id offset $1 limit $2")
            .await?;
        let results = connection.query(&stmt, &[&offset, &limit]).await?;
        Ok(results.iter().map(Group::from).collect())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_group(&self, id: &Uuid) -> Result<Group, Error> {
        if let Some(group) = self.cache.get_group_by_id(id).await {
            return Ok(group);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from groups where id = $1")
            .await?;
        let results = connection.query(&stmt, &[id]).await?;
        let group: Group = results.first().unwrap().into();
        self.cache.cache_group(&group).await;
        Ok(group)
    }

    #[tracing::instrument(skip(self, name))]
    pub async fn get_group_by_name(&self, name: &str) -> Result<Group, Error> {
        if let Some(group) = self.cache.get_group_by_name(name).await {
            return Ok(group);
        }
        let name = name.to_string();
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from groups where name = $1")
            .await?;
        let results = connection.query(&stmt, &[&name]).await?;
        let group: Group = results.first().unwrap().into();
        self.cache.cache_group(&group).await;
        Ok(group)
    }

    #[tracing::instrument(skip(self, offset, limit))]
    pub async fn get_principals(&self, offset: i64, limit: i64) -> Result<Vec<Principal>, Error> {
        let mut principals = Vec::<Principal>::new();
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from principals order by id offset $1 limit $2")
            .await?;
        let results = connection.query(&stmt, &[&offset, &limit]).await?;
        for result in results.iter() {
            principals.push(result.into())
        }
        Ok(principals)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_administrators_group(&self) -> Result<Group, Error> {
        let group = ADMINISTRATORS_GROUP.to_string();
        self.get_group_by_name(&group).await
    }

    // pub async fn get_workflow_manager_group(&self) -> Result<Group, Error> {
    //     let group = WORKFLOW_MANAGERS_GROUP.to_string();
    //     self.get_group_by_name(&group).await
    // }

    #[tracing::instrument(skip(self))]
    pub async fn get_service_account_group(&self) -> Result<Group, Error> {
        self.get_group_by_name(SERVICE_ACCOUNT_GROUP).await
    }

    #[tracing::instrument(skip(self, principal))]
    pub fn new_token(&self, principal: &Principal) -> Result<Token, jsonwebtoken::errors::Error> {
        self.jwt.new_token(principal)
    }

    #[tracing::instrument(skip(self, principal))]
    pub fn new_refresh_token(
        &self,
        principal: &Principal,
    ) -> Result<RefreshToken, jsonwebtoken::errors::Error> {
        self.jwt.new_refresh_token(principal).map(|t| RefreshToken { token: t })
    }

    #[tracing::instrument(skip(self, refresh_token))]
    pub async fn validate_refresh_token(&self, refresh_token: &str) -> Result<Option<Uuid>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from principal_refresh_tokens where token = $1 returning principal_id, expires")
            .await?;
        let token = refresh_token.to_string();
        let results = connection.query(&stmt, &[&token]).await?;
        if let Some(result) = results.first() {
            let expires: DateTime<Utc> = result.get("expires");
            if expires > Utc::now() {
                warn!("refresh token expired");
                let principal_id: Uuid = result.get("principal_id");
                return Ok(Some(principal_id));
            }
        }
        Ok(None)
    }

    #[tracing::instrument(skip(self, principal, refresh_token))]
    pub async fn add_refresh_token(
        &self,
        principal: &Principal,
        refresh_token: &RefreshToken,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "insert into principal_refresh_tokens (token, principal_id) values ($1, $2)",
            )
            .await?;
        let token = refresh_token.token.to_string();
        connection.execute(&stmt, &[&token, &principal.id]).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn expire_refresh_tokens(&self) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from principal_refresh_tokens where expires <= now()")
            .await?;
        connection.execute(&stmt, &[]).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, verified, attributes, credential, groups))]
    pub async fn add_principal(
        &self,
        verified: bool,
        attributes: Value,
        credential: &Credential,
        groups: &Vec<&Uuid>,
    ) -> Result<Uuid, Error> {
        let verification_token = if verified {
            None
        } else {
            Some(hex::encode(Uuid::new_v4().as_bytes()))
        };
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into principals (verified, verification_token, anonymous, attributes) values ($1, $2, false, $3) returning id").await?;
        let results = txn
            .query(&stmt, &[&verified, &verification_token, &attributes])
            .await?;
        if results.is_empty() {
            return Err(Error::new("failed to create principal"));
        }
        let id = results[0].get("id");
        drop(stmt);
        let attributes = credential.get_attributes();
        let stmt = txn.prepare_cached("insert into principal_credentials (principal, type, attributes) values ($1, $2, $3)").await?;
        txn.execute(&stmt, &[&id, &credential.get_type(), &attributes])
            .await?;
        let stmt = txn
            .prepare_cached("insert into principal_groups (principal, group_id) values ($1, $2)")
            .await?;
        for group in groups {
            txn.execute(&stmt, &[&id, group]).await?;
        }
        txn.commit().await?;
        Ok(id)
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_principal_credentials(
        &self,
        id: &Uuid,
    ) -> Result<Vec<Credential>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from principal_credentials where principal = $1")
            .await?;
        let results = connection.query(&stmt, &[id]).await?;
        let mut credentials = Vec::<Credential>::new();
        for result in results.iter() {
            let type_ = result.get("type");
            let attributes = result.get("attributes");
            let credential = match type_ {
                CredentialType::Password => Credential::Password(PasswordCredential::new_from_attributes(attributes)),
                CredentialType::Oauth2 => return Err(Error::new("unsupported")),
            };
            credentials.push(credential);
        }
        Ok(credentials)
    }

    #[tracing::instrument(skip(self, id, credential))]
    pub async fn set_principal_credential(
        &self,
        id: &Uuid,
        credential: &Credential,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update principal_credentials set attributes = $1 where principal = $2")
            .await?;
        let attributes = credential.get_attributes();
        txn.execute(&stmt, &[&attributes, id]).await?;
        txn.commit().await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, attributes, groups))]
    pub async fn add_anonymous_principal<'a>(
        &'a self,
        attributes: Value,
        groups: &'a Vec<&Uuid>,
    ) -> Result<Uuid, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "insert into principals (anonymous, attributes) values (true, $1) returning id",
            )
            .await?;
        let results = txn.query(&stmt, &[&attributes]).await?;
        if results.is_empty() {
            return Err(Error::new("failed to create principal"));
        }
        let id = results[0].get("id");
        drop(stmt);
        let stmt = txn
            .prepare_cached("insert into principal_groups (principal, group_id) values ($1, $2)")
            .await?;
        for group in groups {
            txn.execute(&stmt, &[&id, group]).await?;
        }
        txn.commit().await?;
        Ok(id)
    }

    #[tracing::instrument(skip(self, principal, group))]
    pub async fn add_principal_group(&self, principal: &Uuid, group: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("insert into principal_groups (principal, group_id) values ($1, $2)")
            .await?;
        connection.execute(&stmt, &[principal, group]).await?;
        self.cache.evict_principal(principal).await;
        Ok(())
    }

    #[tracing::instrument(skip(self, principal, group))]
    pub async fn remove_principal_group(
        &self,
        principal: &Uuid,
        group: &Uuid,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from principal_groups where principal = $1 and group_id = $2")
            .await?;
        connection.execute(&stmt, &[principal, group]).await?;
        self.cache.evict_principal(principal).await;
        Ok(())
    }

    #[tracing::instrument(skip(self, connection, principal))]
    pub async fn get_principal_groups(
        &self,
        connection: &Object,
        principal: &Uuid,
    ) -> Result<Vec<Group>, Error> {
        let mut groups = Vec::<Group>::new();
        let stmt = connection.prepare_cached("select g.* from principal_groups as pg inner join groups as g on (pg.group_id = g.id) where principal = $1").await?;
        let results = connection.query(&stmt, &[principal]).await?;
        for result in results.iter() {
            groups.push(result.into())
        }
        Ok(groups)
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_principal_by_id(&self, id: &Uuid) -> Result<Principal, Error> {
        if let Some(principal) = self.cache.get_principal_by_id(id).await {
            return Ok(principal);
        }
        let connection = self.pool.get().await?;
        let principal = self.get_principal_by_id_internal(&connection, id).await?;
        Ok(principal)
    }

    #[tracing::instrument(skip(self, connection, id))]
    async fn get_principal_by_id_internal(
        &self,
        connection: &Object,
        id: &Uuid,
    ) -> Result<Principal, Error> {
        let stmt = connection
            .prepare_cached("select * from principals where id = $1")
            .await?;
        let results = connection.query(&stmt, &[&id]).await?;
        if results.is_empty() {
            return Err(Error::new("invalid principal"));
        }
        let mut principal: Principal = results.first().unwrap().into();
        drop(stmt);
        let groups = self.get_principal_groups(connection, id).await?;
        principal.set_groups(&Some(groups));
        self.cache.cache_principal(&principal).await;
        Ok(principal)
    }

    #[tracing::instrument(skip(self, identifier))]
    pub async fn get_principal_by_identifier(&self, identifier: &str) -> Result<Principal, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select principal from principal_credentials where attributes->>'identifier' = $1",
            )
            .await?;
        let id = identifier.to_lowercase();
        let results = connection.query(&stmt, &[&id]).await?;
        if results.is_empty() {
            return Err(Error::new("invalid credential"));
        }
        let id: Uuid = results.first().unwrap().get("principal");
        drop(results);
        drop(stmt);
        self.get_principal_by_id_internal(&connection, &id).await
    }

    #[tracing::instrument(skip(self, identifier, password))]
    pub async fn get_principal_by_password(
        &self,
        identifier: &str,
        password: &str,
    ) -> Result<Principal, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select principal, attributes->>'password' as hash from principal_credentials where attributes->>'identifier' = $1").await?;
        let id = String::from(identifier);
        let results = connection.query(&stmt, &[&id]).await?;
        if results.is_empty() {
            return Err(Error::new("invalid credential"));
        }
        let result = results.first().unwrap();
        let id: Uuid = result.get("principal");
        let hash: String = result.get("hash");
        drop(results);
        drop(stmt);
        if !verify(&hash, password)? {
            return Err(Error::new("invalid credential"));
        }
        self.get_principal_by_id_internal(&connection, &id).await
    }

    #[tracing::instrument(skip(self, credential, password))]
    pub fn verify_password(
        &self,
        credential: &Credential,
        password: &str,
    ) -> Result<bool, Error> {
        let attrs = credential.get_attributes();
        let hash = attrs.get("password").unwrap().as_str().unwrap();
        Ok(verify(hash, password)?)
    }

    #[tracing::instrument(skip(self, verification_token))]
    pub async fn set_principal_verified(&self, verification_token: &str) -> Result<Uuid, Error> {
        let connection = self.pool.get().await?;
        let token = verification_token.to_string();
        let stmt = connection
            .prepare_cached("update principals set verification_token = null, verified = true where verification_token = $1 returning id")
            .await?;
        let results = connection.query(&stmt, &[&token]).await?;
        if results.is_empty() {
            return Err(Error::new("invalid token"));
        }
        let id: Uuid = results.first().unwrap().get("id");
        self.cache.evict_principal(&id).await;
        Ok(id)
    }

    #[tracing::instrument(skip(self, token))]
    pub async fn get_principal_by_token(&self, token: &str) -> Result<Principal, Error> {
        let claims = self.jwt.validate_token(token)?;
        let id = Uuid::parse_str(claims.sub.as_str())?;
        let principal = self.get_principal_by_id(&id).await?;
        Ok(principal)
    }

    #[tracing::instrument(skip(self, cookie))]
    pub async fn get_principal_by_cookie(&self, cookie: &str) -> Result<Principal, Error> {
        let claims = self.jwt.validate_token(cookie)?;
        let id = Uuid::parse_str(claims.sub.as_str())?;
        let principal = self.get_principal_by_id(&id).await?;
        Ok(principal)
    }
}
