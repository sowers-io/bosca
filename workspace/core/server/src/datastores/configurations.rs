use std::collections::HashMap;
use std::str::FromStr;
use crate::datastores::notifier::Notifier;
use async_graphql::Error;
use deadpool_postgres::GenericClient;
use std::sync::Arc;
use std::sync::atomic::AtomicI64;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use chrono::Utc;
use log::error;
use rand::Rng;
use redis::AsyncCommands;
use sha2::{Digest, Sha256};
use serde_json::Value;
use tokio::sync::RwLock;
use uuid::Uuid;
use bosca_database::TracingPool;
use crate::models::configuration::configuration::{Configuration, ConfigurationInput};
use crate::models::security::permission::Permission;
use crate::redis::RedisClient;

#[derive(Clone)]
pub struct ConfigurationDataStore {
    pool: TracingPool,
    notifier: Arc<Notifier>,
    security_key: String,
    redis: RedisClient,
    last_read: Arc<AtomicI64>,
    cache: Arc<RwLock<HashMap<String, Value>>>,
}

impl ConfigurationDataStore {
    pub fn new(pool: TracingPool, security_key: String, redis: RedisClient, notifier: Arc<Notifier>) -> Self {
        Self {
            pool,
            notifier,
            security_key,
            redis,
            last_read: Arc::new(AtomicI64::new(Utc::now().timestamp())),
            cache: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_configurations(&self) -> Result<Vec<Configuration>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from configurations order by key asc")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, key))]
    pub async fn get_configuration_by_key(&self, key: &str) -> Result<Option<Configuration>, Error> {
        let connection = self.pool.get().await?;
        let key = key.to_string();
        let stmt = connection
            .prepare_cached("select * from configurations where key = $1")
            .await?;
        let rows = connection.query(&stmt, &[&key]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_configuration_by_id(&self, id: &Uuid) -> Result<Option<Configuration>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from configurations where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_permissions(&self, id: &Uuid) -> Result<Vec<Permission>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from configuration_permissions where entity_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, configuration))]
    pub async fn set_configuration(&self, configuration: &ConfigurationInput) -> Result<Uuid, Error> {
        let json = configuration.value.to_string();
        let (value, nonce) = self.encrypt_value(&json)?;
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into configurations (key, description, public) values ($1, $2, $3) on conflict (key) do update set description = $2, public = $3 returning id").await?;
        let result = txn.query_one(&stmt, &[&configuration.key, &configuration.description, &configuration.public]).await?;
        let id: Uuid = result.get("id");
        let stmt = txn.prepare_cached("delete from configuration_values where configuration_id = $1").await?;
        txn.execute(&stmt, &[&id]).await?;
        let stmt = txn.prepare_cached("delete from configuration_permissions where entity_id = $1").await?;
        txn.execute(&stmt, &[&id]).await?;
        let stmt = txn.prepare_cached("insert into configuration_permissions (entity_id, group_id, action) values ($1, $2, $3)").await?;
        for permission in configuration.permissions.iter() {
            let group_id = Uuid::parse_str(&permission.group_id)?;
            txn.execute(&stmt, &[&id, &group_id, &permission.action]).await?;
        }
        let stmt = txn.prepare_cached("insert into configuration_values (configuration_id, value, nonce) values ($1, $2, $3)").await?;
        txn.execute(&stmt, &[&id, &value, &nonce]).await?;
        txn.commit().await?;
        let id_str = id.to_string();
        {
            let mut cache = self.cache.write().await;
            cache.remove(&configuration.key);
        }
        self.update_cache_time().await;
        self.notifier.configuration_changed(&id_str).await?;
        Ok(id)
    }

    async fn update_cache_time(&self) {
        let Ok(mut mgr) = self.redis.get_manager().await else {
            return;
        };
        if let Err(e) = mgr.set::<&str, i64, ()>("bosca:configuration:cache", Utc::now().timestamp()).await {
            error!("Failed to update cache time: {}", e);
        }
    }

    async fn get_cache_time(&self) -> i64 {
        let Ok(mut mgr) = self.redis.get_manager().await else {
            return 0;
        };
        if let Ok(Some(time)) = mgr.get("bosca:configuration:cache").await {
            return time;
        }
        0
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn delete_configuration(&self, id: &Uuid) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("delete configurations where id = $1 returning key").await?;
        let result = txn.query_one(&stmt, &[&id]).await?;
        if result.is_empty() {
            return Ok(())
        }
        let key: String = result.get("key");
        {
            let mut cache = self.cache.write().await;
            cache.remove(&key);
        }
        self.update_cache_time().await;
        let id = id.to_string();
        self.notifier.configuration_changed(&id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, key))]
    pub async fn get_configuration_value(&self, key: &str) -> Result<Option<Value>, Error> {
        {
            let time = self.get_cache_time().await;
            if time > self.last_read.load(std::sync::atomic::Ordering::Relaxed) {
                self.last_read.store(Utc::now().timestamp(), std::sync::atomic::Ordering::Relaxed);
                self.cache.write().await.clear();
            } else {
                let cache = self.cache.read().await;
                let value = cache.get(key);
                if let Some(value) = value {
                    return Ok(Some(value.clone()))
                }
            }
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from configuration_values v inner join configurations c on v.configuration_id = c.id where c.key = $1")
            .await?;
        let rows = connection.query(&stmt, &[&key]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let row = rows.first().unwrap();
        let value: Vec<u8> = row.get("value");
        let nonce: Vec<u8> = row.get("nonce");
        let json_value = self.decrypt_value(&value, &nonce)?;
        let json = Value::from_str(&json_value)?;
        let mut cache = self.cache.write().await;
        cache.insert(key.to_string(), json.clone());
        Ok(Some(json))
    }

    #[tracing::instrument(skip(self))]
    fn derive_key(&self) -> Key<Aes256Gcm> {
        let mut hasher = Sha256::new();
        hasher.update(&self.security_key);
        let result = hasher.finalize();
        Key::<Aes256Gcm>::from_slice(&result[0..32]).to_owned()
    }

    #[tracing::instrument(skip(self, plaintext))]
    fn encrypt_value(&self, plaintext: &str) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let key = self.derive_key();
        let cipher = Aes256Gcm::new(&key);
        let mut rng = rand::rng();
        let binding: [u8; 12] = rng.random();
        let nonce = Nonce::from_slice(&binding);
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|err| Error::new(format!("Encryption failed: {}", err)))?;
        Ok((ciphertext, nonce.to_vec()))
    }

    #[tracing::instrument(skip(self, ciphertext, nonce))]
    fn decrypt_value(&self, ciphertext: &[u8], nonce: &[u8]) -> Result<String, Error> {
        let key = self.derive_key();
        let cipher = Aes256Gcm::new(&key);
        let nonce = Nonce::from_slice(nonce);
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|err| Error::new(format!("Decryption failed: {}", err)))?;
        String::from_utf8(plaintext).map_err(|err| Error::new(format!("Invalid UTF-8: {}", err)))
    }
}
