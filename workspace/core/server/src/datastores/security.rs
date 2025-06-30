use crate::context::BoscaContext;
use crate::datastores::cache::manager::BoscaCacheManager;
use crate::datastores::configurations::ConfigurationDataStore;
use crate::datastores::security_cache::SecurityCache;
use crate::models::configuration::configuration::ConfigurationInput;
use crate::models::content::collection::{CollectionInput, CollectionType};
use crate::models::profiles::profile::ProfileInput;
use crate::models::profiles::profile_attribute::ProfileAttributeInput;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use crate::models::security::credentials::{Credential, CredentialType};
use crate::models::security::credentials_oauth2::Oauth2Credential;
use crate::models::security::credentials_password::PasswordCredential;
use crate::models::security::credentials_scrypt::PasswordScryptCredential;
use crate::models::security::group::Group;
use crate::models::security::group_type::GroupType;
use crate::models::security::permission::{Permission, PermissionAction};
use crate::models::security::principal::Principal;
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::security::account::{
    Account, FacebookPicture, FacebookPictureData, FacebookUser, GoogleAccount,
};
use crate::security::firebase::{FirebaseImportUser, FirebaseImportUsers, HashConfig};
use crate::security::jwt::Jwt;
use crate::security::token::Token;
use crate::util::signed_url::{sign_url, verify_signed_url};
use crate::workflow::core_workflow_ids::PROFILE_UPDATE_STORAGE;
use async_graphql::*;
use bosca_database::TracingPool;
use chrono::{DateTime, Utc};
use deadpool_postgres::{GenericClient, Object, Transaction};
use firebase_scrypt::FirebaseScrypt;
use log::{error, info, warn};
use oauth2::basic::BasicTokenType;
use oauth2::{EmptyExtraTokenFields, StandardTokenResponse};
use serde_json::{json, Value};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinSet;
use uuid::Uuid;

#[derive(Clone)]
pub struct SecurityDataStore {
    cache: SecurityCache,
    pool: TracingPool,
    jwt: Jwt,
    url_secret_key: String,
    firebase_scrypt: Arc<RwLock<Option<FirebaseScrypt>>>,
    pub auto_verify_accounts: bool,
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
    pub token: String,
}

impl Debug for RefreshToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RefreshToken").finish()
    }
}

impl SecurityDataStore {
    pub async fn new(
        cfg: ConfigurationDataStore,
        cache: &mut BoscaCacheManager,
        pool: TracingPool,
        jwt: Jwt,
        url_secret_key: String,
        auto_verify_accounts: bool,
    ) -> Result<Self, Error> {
        let firebase = if let Some(value) = cfg
            .get_configuration_value("firebase.security.hash")
            .await?
        {
            let cfg: HashConfig = serde_json::from_value(value)?;
            Some(FirebaseScrypt::new(
                &cfg.base64_salt_separator,
                &cfg.base64_signer_key,
                cfg.rounds,
                cfg.mem_cost,
            ))
        } else {
            None
        };
        Ok(Self {
            cache: SecurityCache::new(cache).await?,
            pool,
            jwt,
            url_secret_key,
            auto_verify_accounts,
            firebase_scrypt: Arc::new(RwLock::new(firebase)),
        })
    }

    #[tracing::instrument(skip(self, ctx, config))]
    pub async fn set_firebase_hash_config(
        &self,
        ctx: &BoscaContext,
        config: HashConfig,
    ) -> Result<(), Error> {
        let cfg = ConfigurationInput {
            key: "firebase.security.hash".to_string(),
            description: "Firebase Security Hash".to_string(),
            value: serde_json::to_value(&config)?,
            public: false,
            permissions: vec![],
        };
        ctx.configuration.set_configuration(&cfg).await?;
        self.firebase_scrypt
            .write()
            .await
            .replace(FirebaseScrypt::new(
                &config.base64_salt_separator,
                &config.base64_signer_key,
                config.rounds,
                config.mem_cost,
            ));
        Ok(())
    }

    #[tracing::instrument(skip(self, url))]
    pub fn sign_url(&self, url: &str) -> String {
        sign_url(url, &self.url_secret_key, 3600)
    }

    #[tracing::instrument(skip(self, url))]
    pub fn verify_signed_url(&self, url: &str) -> bool {
        verify_signed_url(url, &self.url_secret_key)
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn create_verification_token(&self, id: &Uuid) -> Result<String, Error> {
        let verification_token = hex::encode(Uuid::new_v4().as_bytes());
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update principals set verification_token = $1 where id = $2")
            .await?;
        txn.execute(&stmt, &[&verification_token, id]).await?;
        txn.commit().await?;
        Ok(verification_token)
    }

    #[tracing::instrument(skip(self, name, description, group_type))]
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
        let group = Group::new(id, name.clone(), description.clone(), group_type);
        self.cache.cache_group(&group).await;
        Ok(group)
    }

    #[tracing::instrument(skip(self, id, name, description, group_type))]
    pub async fn edit_group(
        &self,
        id: &Uuid,
        name: &String,
        description: &String,
        group_type: GroupType,
    ) -> Result<Group, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update groups set name = $1, description = $2, type = $3::group_type where id = $4")
            .await?;
        let results = connection
            .query(&stmt, &[name, description, &group_type, id])
            .await?;
        let id: Uuid = results.first().unwrap().get(0);
        let group = Group::new(id, name.clone(), description.clone(), group_type);
        self.cache.cache_group(&group).await;
        Ok(group)
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn delete_group(
        &self,
        id: &Uuid,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from groups where id = $4")
            .await?;
        connection
            .execute(&stmt, &[id])
            .await?;
        self.cache.evict_group(id).await;
        Ok(())
    }

    // #[tracing::instrument(skip(self, txn, name, description, group_type))]
    // pub async fn add_group_txn(
    //     &self,
    //     txn: &Transaction<'_>,
    //     name: &String,
    //     description: &String,
    //     group_type: GroupType,
    // ) -> Result<Group, Error> {
    //     let stmt = txn
    //         .prepare_cached("insert into groups (name, description, type) values ($1, $2, $3::group_type) returning id")
    //         .await?;
    //     let results = txn
    //         .query(&stmt, &[name, description, &group_type])
    //         .await?;
    //     let id: Uuid = results.first().unwrap().get(0);
    //     let group = Group::new(id, name.clone(), group_type);
    //     Ok(group)
    // }

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
        self.jwt
            .new_refresh_token(principal)
            .map(|t| RefreshToken { token: t })
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
                let principal_id: Uuid = result.get("principal_id");
                return Ok(Some(principal_id));
            } else {
                warn!("refresh token expired");
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

    // #[tracing::instrument(skip(self, verified, attributes, credential, groups))]
    // pub async fn add_principal(
    //     &self,
    //     verified: Option<bool>,
    //     attributes: Value,
    //     credential: &Credential,
    //     groups: &Vec<&Uuid>,
    // ) -> Result<Uuid, Error> {
    //     let mut connection = self.pool.get().await?;
    //     let txn = connection.transaction().await?;
    //     let id = self.add_principal_txn(&txn, verified, attributes, credential, groups).await?;
    //     txn.commit().await?;
    //     Ok(id)
    // }

    #[tracing::instrument(skip(self, txn, verified, attributes, credential, groups))]
    pub async fn add_principal_txn(
        &self,
        txn: &Transaction<'_>,
        verified: Option<bool>,
        attributes: Value,
        credential: &Credential,
        groups: &Vec<&Uuid>,
    ) -> Result<Uuid, Error> {
        let verified = verified.unwrap_or(self.auto_verify_accounts);
        let verification_token = if verified {
            None
        } else {
            Some(hex::encode(Uuid::new_v4().as_bytes()))
        };
        let stmt = txn.prepare_cached("insert into principals (verified, verification_token, anonymous, attributes) values ($1, $2, false, $3) returning id").await?;
        let results = txn
            .query(&stmt, &[&verified, &verification_token, &attributes])
            .await?;
        if results.is_empty() {
            return Err(Error::new("failed to create principal"));
        }
        let id = results[0].get("id");
        let attributes = credential.get_attributes();
        let stmt = txn.prepare_cached("insert into principal_credentials (principal, type, attributes) values ($1, $2, $3)").await?;
        txn.execute(&stmt, &[&id, &credential.get_type(), &attributes]).await?;
        if !groups.is_empty() {
            let stmt = txn
                .prepare_cached("insert into principal_groups (principal, group_id) values ($1, $2)")
                .await?;
            for group in groups {
                txn.execute(&stmt, &[&id, group]).await?;
            }
        }
        Ok(id)
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_principal_credentials(&self, id: &Uuid) -> Result<Vec<Credential>, Error> {
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
                CredentialType::Password => {
                    Credential::Password(PasswordCredential::new_from_attributes(attributes))
                }
                CredentialType::PasswordScrypt => {
                    if let Some(firebase) = self.firebase_scrypt.read().await.clone() {
                        Credential::PasswordScrypt(PasswordScryptCredential::new_from_attributes(
                            firebase, attributes,
                        ))
                    } else {
                        return Err(Error::new("missing scrypt configuration"));
                    }
                }
                CredentialType::Oauth2 => {
                    Credential::Oauth2(Oauth2Credential::new_from_attributes(attributes))
                }
            };
            credentials.push(credential);
        }
        Ok(credentials)
    }

    #[tracing::instrument(skip(self, id, attributes))]
    pub async fn merge_principal_attributes(
        &self,
        id: &Uuid,
        attributes: Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update principals set attributes = coalesce(attributes, '{}'::jsonb) || $1 where id = $2")
            .await?;
        txn.execute(&stmt, &[&attributes, id]).await?;
        txn.commit().await?;
        Ok(())
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
            .prepare_cached("update principal_credentials set attributes = $1 where principal = $2 and type = $3 and attributes->>'identifier' = $4")
            .await?;
        let attributes = credential.get_attributes();
        txn.execute(&stmt, &[&attributes, id, &credential.get_type(), &credential.identifier()]).await?;
        txn.commit().await?;
        Ok(())
    }

    pub async fn add_principal_credential(
        &self,
        id: &Uuid,
        credential: &Credential,
    ) -> Result<(), Error> {
        let attributes = credential.get_attributes();
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into principal_credentials (principal, type, attributes) values ($1, $2, $3)").await?;
        txn.execute(&stmt, &[&id, &credential.get_type(), &attributes]).await?;
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

    // #[tracing::instrument(skip(self, txn, principal, group))]
    // pub async fn add_principal_group_txn(&self, txn: &Transaction<'_>, principal: &Uuid, group: &Uuid) -> Result<(), Error> {
    //     let stmt = txn
    //         .prepare_cached("insert into principal_groups (principal, group_id) values ($1, $2)")
    //         .await?;
    //     txn.execute(&stmt, &[principal, group]).await?;
    //     self.cache.evict_principal(principal).await;
    //     Ok(())
    // }

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

    #[tracing::instrument(skip(self, principal))]
    pub async fn get_principal_groups(&self, principal: &Uuid) -> Result<Vec<Uuid>, Error> {
        if let Some(principal) = self.cache.get_principal_group_ids(principal).await {
            return Ok(principal);
        }
        let connection = self.pool.get().await?;
        let mut groups = Vec::<Uuid>::new();
        let stmt = connection
            .prepare_cached("select group_id from principal_groups where principal = $1")
            .await?;
        let results = connection.query(&stmt, &[principal]).await?;
        for result in results.iter() {
            groups.push(result.get("group_id"));
        }
        self.cache
            .cache_principal_group_ids(principal, groups.clone())
            .await;
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
        let principal: Principal = results.first().unwrap().into();
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
        self.get_principal_by_id_internal(&connection, &id).await
    }

    #[tracing::instrument(skip(self, identifier, oauth2_type))]
    pub async fn get_principal_by_identifier_oauth2(
        &self,
        identifier: &str,
        oauth2_type: &str,
    ) -> Result<Principal, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select principal from principal_credentials where attributes->>'identifier' = $1 and attributes->>'type' = $2",
            )
            .await?;
        let id = identifier.to_lowercase();
        let oauth2_type = oauth2_type.to_lowercase();
        let results = connection.query(&stmt, &[&id, &oauth2_type]).await?;
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
        let (id, credential_type, attributes) = {
            let stmt = connection.prepare_cached("select principal, type, attributes from principal_credentials where attributes->>'identifier' = $1").await?;
            let id = String::from(identifier);
            let results = connection.query(&stmt, &[&id]).await?;
            if results.is_empty() {
                return Err(Error::new("invalid credential"));
            }
            let result = results.first().unwrap();
            let id: Uuid = result.get("principal");
            let credential_type: CredentialType = result.get("type");
            let attributes: Value = result.get("attributes");
            (id, credential_type, attributes)
        };
        let credential = match credential_type {
            CredentialType::Password => {
                Credential::Password(PasswordCredential::new_from_attributes(attributes))
            }
            CredentialType::PasswordScrypt => {
                Credential::PasswordScrypt(PasswordScryptCredential::new_from_attributes(
                    self.firebase_scrypt.read().await.clone().unwrap(),
                    attributes,
                ))
            }
            CredentialType::Oauth2 => return Err(Error::new("invalid credential")),
        };
        if !credential.verify(password)? {
            return Err(Error::new("invalid credential"));
        }
        self.get_principal_by_id_internal(&connection, &id).await
    }

    #[tracing::instrument(skip(self, credential, password))]
    pub fn verify_password(&self, credential: &Credential, password: &str) -> Result<bool, Error> {
        credential.verify(password)
    }

    /// verify forgot password token, so, verified must be true for this to work
    #[tracing::instrument(skip(self, token))]
    pub async fn verify_verification_token(&self, token: &str) -> Result<Principal, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from principals where verification_token = $1 and verified = true",
            )
            .await?;
        let token = token.to_string();
        let results = connection.query(&stmt, &[&token]).await?;
        if results.is_empty() {
            return Err(Error::new("invalid token"));
        }
        Ok(results.first().unwrap().into())
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

    #[tracing::instrument(skip(ctx, users))]
    pub async fn import_firebase_users(&self, ctx: &BoscaContext, users: &FirebaseImportUsers) -> Result<(), Error> {
        let Some(firebase_scrypt) = self.firebase_scrypt.read().await.clone() else {
            return Err(Error::new("missing scrypt configuration"));
        };
        let mut u = Vec::new();
        let mut set = JoinSet::new();
        let mut ix = 0;
        for user in users.users.iter() {
            u.push(user.clone());
            if u.len() == 5000 {
                let ctx = ctx.clone();
                let firebase_scrypt = firebase_scrypt.clone();
                ix += 1;
                let ix = ix;
                set.spawn(async move {
                    if let Err(e) = ctx.security.import_firebase_users_batch(ix, &ctx, &firebase_scrypt, u).await {
                        error!("failed to import firebase users: {e:?}");
                    }
                });
                u = Vec::new();
            }
        }
        if !u.is_empty() {
            let ctx = ctx.clone();
            let firebase_scrypt = firebase_scrypt.clone();
            ix += 1;
            set.spawn(async move {
                if let Err(e) = ctx.security.import_firebase_users_batch(ix, &ctx, &firebase_scrypt, u).await {
                    error!("failed to import firebase users: {e:?}");
                }
            });
        }
        set.join_all().await;
        Ok(())
    }

    async fn import_firebase_users_batch(&self, ix: i32, ctx: &BoscaContext, firebase_scrypt: &FirebaseScrypt, users: Vec<FirebaseImportUser>) -> Result<(), Error> {
        info!("importing firebase users batch {ix}...");
        let mut connection = self.pool.get().await?;
        let mut txn = connection.transaction().await?;
        let mut ids = Vec::<Uuid>::new();
        let mut count = 0;
        for user in users {
            if self.get_principal_by_identifier(&user.email).await.is_ok() {
                continue;
            }
            if self.get_principal_by_identifier(&user.local_id).await.is_ok() {
                continue;
            }
            let mut found = false;
            for p in &user.provider_user_info {
                if self.get_principal_by_identifier(&p.raw_id).await.is_ok() {
                    found = true;
                    break;
                }
            }
            if found {
                continue;
            }
            let result = self.import_firebase_user(&txn, ctx, firebase_scrypt, &user).await;
            match result {
                Ok(Some(id)) => {
                    ids.push(id);
                }
                Ok(None) => {}
                Err(e) => {
                    error!("failed to import firebase user: {e:?}");
                }
            }
            count += 1;
            if  count > 1 {
                if count % 50 == 0 {
                    info!("processed {ix} {count} users");
                }
                if count % 500 == 0 {
                    txn.commit().await?;
                    txn = connection.transaction().await?;
                }
            }
        }
        txn.commit().await?;
        info!("finished firebase user import");
        for profile_id in ids {
            let mut request = EnqueueRequest {
                workflow_id: Some(PROFILE_UPDATE_STORAGE.to_string()),
                profile_id: Some(profile_id),
                ..Default::default()
            };
            ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
        }
        Ok(())
    }

    async fn import_firebase_user(&self, txn: &Transaction<'_>, ctx: &BoscaContext, firebase_scrypt: &FirebaseScrypt, user: &FirebaseImportUser) -> Result<Option<Uuid>, Error> {
        if user.email.is_empty() {
            return Ok(None);
        }
        let name = if user.display_name.is_empty() {
            "User".to_string()
        } else {
            user.display_name.clone()
        };
        let profile = ProfileInput {
            slug: None,
            name: name.clone(),
            visibility: ProfileVisibility::User,
            attributes: vec![
                ProfileAttributeInput {
                    id: None,
                    type_id: "bosca.profiles.email".to_string(),
                    visibility: ProfileVisibility::User,
                    confidence: 100,
                    priority: 1,
                    source: "firebase".to_string(),
                    attributes: Some(json!({"email": user.email.clone()})),
                    metadata_id: None,
                    metadata_supplementary: None,
                    expiration: None,
                },
                ProfileAttributeInput {
                    id: None,
                    type_id: "bosca.profiles.name".to_string(),
                    visibility: ProfileVisibility::User,
                    confidence: 100,
                    priority: 1,
                    source: "firebase".to_string(),
                    attributes: Some(json!({"name": name})),
                    metadata_id: None,
                    metadata_supplementary: None,
                    expiration: None,
                },
            ],
        };
        let profile_id = if user.provider_user_info.is_empty() {
            let credential = PasswordScryptCredential::new_with_hash_and_salt(
                firebase_scrypt.clone(),
                &user.email,
                &user.local_id,
                &user.salt,
                &user.password_hash,
            );
            let credential = Credential::PasswordScrypt(credential);
            let (_, id) = self.add_principal_with_credential_txn(
                txn,
                ctx,
                &credential,
                &profile,
                if self.auto_verify_accounts {
                    Some(true)
                } else {
                    Some(user.email_verified)
                },
            ).await?;
            Some(id)
        } else {
            let mut account = None;
            for provider_user_info in &user.provider_user_info {
                account = match provider_user_info.provider_id.as_str() {
                    "google.com" => {
                        let google = GoogleAccount {
                            sub: provider_user_info.raw_id.clone(),
                            name: user.display_name.clone(),
                            given_name: "".to_string(),
                            family_name: "".to_string(),
                            picture: provider_user_info.photo_url.clone(),
                            email: user.email.clone(),
                            email_verified: if self.auto_verify_accounts {
                                true
                            } else {
                                user.email_verified
                            },
                            hd: "".to_string(),
                        };
                        Some(Account::new_google(google))
                    }
                    "facebook.com" => {
                        let facebook = FacebookUser {
                            id: provider_user_info.raw_id.clone(),
                            name: user.display_name.clone(),
                            picture: FacebookPicture {
                                data: FacebookPictureData {
                                    url: provider_user_info.photo_url.clone(),
                                    width: 0,
                                    height: 0,
                                    is_silhouette: false,
                                },
                            },
                            email: user.email.clone(),
                        };
                        Some(Account::new_facebook(facebook))
                    }
                    _ => {
                        error!("unsupported provider: {}", provider_user_info.provider_id);
                        None
                    }
                };
            }
            if let Some(account) = account {
                let mut credential = Oauth2Credential::new(
                    &account,
                    None::<&StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>>,
                )?;
                credential.set_attribute("local_id", Value::String(user.local_id.clone()));
                let credential = Credential::Oauth2(credential);
                let (_, id) = self.add_principal_with_credential_txn(
                    txn,
                    ctx,
                    &credential,
                    &profile,
                    if self.auto_verify_accounts {
                        Some(true)
                    } else {
                        Some(user.email_verified)
                    },
                ).await?;
                Some(id)
            } else {
                None
            }
        };
        Ok(profile_id)
    }

    #[tracing::instrument(skip(self, ctx, credential, profile, verified, set_ready))]
    pub async fn add_principal_with_credential(
        &self,
        ctx: &BoscaContext,
        credential: &Credential,
        profile: &ProfileInput,
        verified: Option<bool>,
        set_ready: bool,
        add_collection: bool,
    ) -> std::result::Result<(Uuid, Uuid), Error> {
        let (principal_id, profile_id, collection_id) = {
            let mut connection = self.pool.get().await?;
            let txn = connection.transaction().await?;

            let groups = vec![];
            let principal_id = ctx
                .security
                .add_principal_txn(
                    &txn,
                    verified,
                    Value::Null,
                    credential,
                    &groups,
                )
                .await?;

            let collection_id = if add_collection {
                let collection_name = format!("Collection for {principal_id}");
                Some(ctx
                    .content
                    .collections
                    .add_txn(
                        &txn,
                        &CollectionInput {
                            name: collection_name,
                            collection_type: Some(CollectionType::System),
                            trait_ids: Some(vec!["profile".to_string()]),
                            ..Default::default()
                        },
                        false,
                    )
                    .await?.0)
            } else {
                None
            };

            let profile_id = ctx
                .profile
                .add_txn(&txn, Some(principal_id), profile, collection_id)
                .await?;

            txn.commit().await?;

            (principal_id, profile_id, collection_id)
        };

        if let Some(collection_id) = collection_id {
            let group_name = format!("principal.{principal_id}");
            let description = format!("Group for {principal_id}");
            let group = ctx
                .security
                .add_group(&group_name, &description, GroupType::Principal)
                .await?;
            ctx.security
                .add_principal_group(&principal_id, &group.id)
                .await?;

            for action in [
                PermissionAction::View,
                PermissionAction::List,
                PermissionAction::Edit,
            ] {
                let permission = Permission {
                    entity_id: collection_id,
                    group_id: group.id,
                    action,
                };
                ctx.content
                    .collection_permissions
                    .add(ctx, &permission)
                    .await?;
            }

            if set_ready {
                let collection = ctx.content.collections.get(&collection_id).await?.expect("Collection not found");
                let principal = ctx.security.get_principal_by_id(&principal_id).await?;
                ctx.content
                    .collection_workflows
                    .set_ready_and_enqueue(ctx, &principal, &collection, None)
                    .await?;
            }
        }

        Ok((principal_id, profile_id))
    }

    #[tracing::instrument(skip(self, txn, ctx, credential, profile, verified))]
    async fn add_principal_with_credential_txn(
        &self,
        txn: &Transaction<'_>,
        ctx: &BoscaContext,
        credential: &Credential,
        profile: &ProfileInput,
        verified: Option<bool>,
    ) -> std::result::Result<(Uuid, Uuid), Error> {
        let groups = vec![];
        let principal_id = ctx
            .security
            .add_principal_txn(
                txn,
                verified,
                Value::Null,
                credential,
                &groups,
            )
            .await?;
        let profile_id = ctx
            .profile
            .add_txn(txn, Some(principal_id), profile, None)
            .await?;
        Ok((principal_id, profile_id))
    }
}
