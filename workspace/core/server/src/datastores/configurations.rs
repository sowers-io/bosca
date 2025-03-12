use std::collections::HashMap;
use std::str::FromStr;
use crate::datastores::notifier::Notifier;
use async_graphql::Error;
use deadpool_postgres::{GenericClient, Pool};
use std::sync::Arc;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::Rng;
use sha2::{Digest, Sha256};
use serde_json::Value;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::models::configuration::configuration::{Configuration, ConfigurationInput};
use crate::models::security::permission::Permission;

#[derive(Clone)]
pub struct ConfigurationDataStore {
    pool: Arc<Pool>,
    notifier: Arc<Notifier>,
    security_key: String,
    cache: Arc<RwLock<HashMap<String, Value>>>,
}

impl ConfigurationDataStore {
    pub fn new(pool: Arc<Pool>, security_key: String, notifier: Arc<Notifier>) -> Self {
        Self {
            pool,
            notifier,
            security_key,
            cache: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub async fn get_configurations(&self) -> Result<Vec<Configuration>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from configurations order by key asc")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_configuration_by_key(&self, key: &str) -> Result<Option<Configuration>, Error> {
        let connection = self.pool.get().await?;
        let key = key.to_string();
        let stmt = connection
            .prepare_cached("select * from configurations where key = $1")
            .await?;
        let rows = connection.query(&stmt, &[&key]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    pub async fn get_configuration_by_id(&self, id: &Uuid) -> Result<Option<Configuration>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from configurations where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    pub async fn get_permissions(&self, id: &Uuid) -> Result<Vec<Permission>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from configuration_permissions where entity_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

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
            txn.execute(&stmt, &[&id, &permission.group_id, &permission.action]).await?;
        }
        let stmt = txn.prepare_cached("insert into configuration_values (configuration_id, value, nonce) values ($1, $2, $3)").await?;
        txn.execute(&stmt, &[&id, &value, &nonce]).await?;
        txn.commit().await?;
        let id_str = id.to_string();
        {
            let mut cache = self.cache.write().await;
            cache.remove(&configuration.key);
        }
        self.notifier.configuration_changed(&id_str).await?;
        Ok(id)
    }

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
        let id = id.to_string();
        self.notifier.configuration_changed(&id).await?;
        Ok(())
    }

    pub async fn get_configuration_value(&self, key: &str) -> Result<Option<Value>, Error> {
        {
            let cache = self.cache.read().await;
            let value = cache.get(key);
            if let Some(value) = value {
                return Ok(Some(value.clone()))
            }
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from configuration_values v inner join configurations c on v.configuration_id = c.id where c.key = $1")
            .await?;
        let row = connection.query_one(&stmt, &[&key]).await?;
        if row.is_empty() {
            return Ok(None);
        }
        let value: Vec<u8> = row.get("value");
        let nonce: Vec<u8> = row.get("nonce");
        let json_value = self.decrypt_value(&value, &nonce)?;
        let json = Value::from_str(&json_value)?;
        let mut cache = self.cache.write().await;
        cache.insert(key.to_string(), json.clone());
        Ok(Some(json))
    }

    fn derive_key(&self) -> Key<Aes256Gcm> {
        let mut hasher = Sha256::new();
        hasher.update(&self.security_key);
        let result = hasher.finalize();
        Key::<Aes256Gcm>::from_slice(&result[0..32]).to_owned()
    }

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
