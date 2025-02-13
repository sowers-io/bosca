use crate::models::security::credentials::{Credential, CredentialType};
use crate::models::security::group::Group;
use crate::models::security::password::{encrypt, verify};
use crate::models::security::principal::Principal;
use crate::security::jwt::Jwt;
use crate::security::token::Token;
use async_graphql::*;
use deadpool_postgres::{GenericClient, Object, Pool};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;
use crate::util::signed_url::{sign_url, verify_signed_url};

#[derive(Clone)]
pub struct SecurityDataStore {
    pool: Arc<Pool>,
    jwt: Jwt,
    url_secret_key: String,
}

pub const ADMINISTRATORS_GROUP: &str = "administrators";
pub const SERVICE_ACCOUNT_GROUP: &str = "sa";
pub const MODEL_MANAGERS_GROUP: &str = "model.managers";
pub const WORKFLOW_MANAGERS_GROUP: &str = "workflow.managers";

impl SecurityDataStore {
    pub fn new(pool: Arc<Pool>, jwt: Jwt, url_secret_key: String) -> Self {
        Self { pool, jwt, url_secret_key }
    }

    pub fn sign_url(&self, url: &str) -> String {
        sign_url(url, &self.url_secret_key, 3600)
    }

    pub fn verify_signed_url(&self, url: &str) -> bool {
        verify_signed_url(url, &self.url_secret_key)
    }

    #[allow(dead_code)]
    pub async fn add_group(&self, name: &String, description: &String) -> Result<Group, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("insert into groups (name, description) values ($1, $2) returning id")
            .await?;
        let results = connection.query(&stmt, &[name, description]).await?;
        let id: Uuid = results.first().unwrap().get(0);
        Ok(Group::new(id, name.clone()))
    }

    pub async fn get_groups(&self, offset: i64, limit: i64) -> Result<Vec<Group>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from groups order by id offset $1 limit $2").await?;
        let results = connection.query(&stmt, &[&offset, &limit]).await?;
        Ok(results.iter().map(Group::from).collect())
    }

    pub async fn get_group(&self, id: &Uuid) -> Result<Group, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from groups where id = $1")
            .await?;
        let results = connection.query(&stmt, &[id]).await?;
        Ok(results.first().unwrap().into())
    }

    pub async fn get_group_by_name(&self, name: &String) -> Result<Group, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from groups where name = $1")
            .await?;
        let results = connection.query(&stmt, &[name]).await?;
        Ok(results.first().unwrap().into())
    }

    pub async fn get_principals(
        &self,
        offset: i64,
        limit: i64
    ) -> Result<Vec<Principal>, Error> {
        let mut principals = Vec::<Principal>::new();
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from principals order by id offset $1 limit $2").await?;
        let results = connection.query(&stmt, &[&offset, &limit]).await?;
        for result in results.iter() {
            principals.push(result.into())
        }
        Ok(principals)
    }

    pub async fn get_administrators_group(&self) -> Result<Group, Error> {
        let group = ADMINISTRATORS_GROUP.to_string();
        self.get_group_by_name(&group).await
    }

    // pub async fn get_workflow_manager_group(&self) -> Result<Group, Error> {
    //     let group = WORKFLOW_MANAGERS_GROUP.to_string();
    //     self.get_group_by_name(&group).await
    // }

    pub async fn get_service_account_group(&self) -> Result<Group, Error> {
        let group = SERVICE_ACCOUNT_GROUP.to_string();
        self.get_group_by_name(&group).await
    }

    pub fn new_token(&self, principal: &Principal) -> Result<Token, jsonwebtoken::errors::Error> {
        self.jwt.new_token(principal)
    }

    pub async fn add_principal(
        &self,
        verified: bool,
        attributes: Value,
        credential: &impl Credential,
        groups: &Vec<&Uuid>,
    ) -> Result<Uuid, Error> {
        let verification_token = if verified { None } else { Some(hex::encode(Uuid::new_v4().as_bytes())) };
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into principals (verified, verification_token, anonymous, attributes) values ($1, $2, false, $3) returning id").await?;
        let results = txn.query(&stmt, &[&verified, &verification_token, &attributes]).await?;
        if results.is_empty() {
            return Err(Error::new("failed to create principal"));
        }
        let id = results[0].get("id");
        drop(stmt);
        match credential.get_type() {
            CredentialType::Password => {
                let attributes = Value::Object(match credential.get_attributes() {
                    Value::Object(hash) => {
                        let mut m = hash.clone();
                        m.insert(
                            "password".to_string(),
                            Value::String(encrypt(
                                hash.get("password").unwrap().as_str().unwrap().to_string(),
                            )?),
                        );
                        m
                    }
                    _ => return Err(Error::new("missing attributes")),
                });
                let stmt = txn.prepare_cached("insert into principal_credentials (principal, type, attributes) values ($1, $2, $3)").await?;
                txn.execute(&stmt, &[&id, &credential.get_type(), &attributes])
                    .await?;
            }
            CredentialType::Oauth2 => return Err(Error::new("unsupported")),
        }
        let stmt = txn
            .prepare_cached("insert into principal_groups (principal, group_id) values ($1, $2)")
            .await?;
        for group in groups {
            txn.execute(&stmt, &[&id, group]).await?;
        }
        txn.commit().await?;
        Ok(id)
    }

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

    pub async fn add_principal_group(&self, principal: &Uuid, group: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("insert into principal_groups (principal, group_id) values ($1, $2)")
            .await?;
        connection.execute(&stmt, &[principal, group]).await?;
        Ok(())
    }

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

    pub async fn get_principal_by_id(&self, id: &Uuid) -> Result<Principal, Error> {
        let connection = self.pool.get().await?;
        let principal = self.get_principal_by_id_internal(&connection, id).await?;
        Ok(principal)
    }

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
        Ok(principal)
    }

    pub async fn get_principal_by_identifier(&self, identifier: &str) -> Result<Principal, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select principal from principal_credentials where attributes->>'identifier' = $1",
            )
            .await?;
        let id = String::from(identifier);
        let results = connection.query_one(&stmt, &[&id]).await?;
        if results.is_empty() {
            return Err(Error::new("invalid credential"));
        }
        let id: Uuid = results.get("principal");
        drop(results);
        drop(stmt);
        self.get_principal_by_id_internal(&connection, &id).await
    }

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
        Ok(id)
    }

    pub async fn get_principal_by_token(&self, token: &str) -> Result<Principal, Error> {
        let claims = self.jwt.validate_token(token)?;
        let id = Uuid::parse_str(claims.sub.as_str())?;
        let principal = self.get_principal_by_id(&id).await?;
        Ok(principal)
    }

    pub async fn get_principal_by_cookie(&self, cookie: &str) -> Result<Principal, Error> {
        let claims = self.jwt.validate_token(cookie)?;
        let id = Uuid::parse_str(claims.sub.as_str())?;
        let principal = self.get_principal_by_id(&id).await?;
        Ok(principal)
    }
}
