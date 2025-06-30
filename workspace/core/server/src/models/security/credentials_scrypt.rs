use crate::models::security::credentials::{CredentialInterface, CredentialType};
use aes_gcm::aead::OsRng;
use argon2::password_hash::SaltString;
use async_graphql::Error;
use firebase_scrypt::FirebaseScrypt;
use serde_json::Value;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct PasswordScryptCredential {
    pub credential_type: CredentialType,
    pub firebase_scrypt: FirebaseScrypt,
    pub attributes: Value,
}

impl Debug for PasswordScryptCredential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PasswordScryptCredential").finish()
    }
}

impl PasswordScryptCredential {
    // pub fn new(firebase_scrypt: FirebaseScrypt, identifier: String, password: String) -> Result<Self, Error> {
    //     if password.trim().is_empty() {
    //         return Err(Error::from("Password is Required"));
    //     }
    //     let mut map = Map::<String, Value>::new();
    //     map.insert(
    //         "identifier".to_string(),
    //         Value::String(identifier.to_lowercase()),
    //     );
    //     let salt = SaltString::generate(&mut OsRng);
    //     let hash = firebase_scrypt.generate_base64_hash(&password, salt.as_str()).map_err(|e| format!("{:?}", e))?;
    //     map.insert("passwordHash".to_string(), Value::String(hash));
    //     map.insert("salt".to_string(), Value::String(salt.as_str().to_string()));
    //     Ok(Self {
    //         credential_type: CredentialType::Password,
    //         firebase_scrypt,
    //         attributes: Value::Object(map),
    //     })
    // }

    pub fn new_from_attributes(firebase_scrypt: FirebaseScrypt, attributes: Value) -> Self {
        Self {
            credential_type: CredentialType::PasswordScrypt,
            firebase_scrypt,
            attributes,
        }
    }

    pub fn new_with_hash_and_salt(firebase_scrypt: FirebaseScrypt, identifier: &str, local_id: &str, salt: &str, hash: &str) -> Self {
        let mut map = serde_json::Map::new();
        map.insert("identifier".to_string(), Value::String(identifier.to_string()));
        map.insert("local_id".to_string(), Value::String(local_id.to_string()));
        map.insert("passwordHash".to_string(), Value::String(hash.to_string()));
        map.insert("salt".to_string(), Value::String(salt.to_string()));
        Self {
            credential_type: CredentialType::PasswordScrypt,
            firebase_scrypt,
            attributes: Value::Object(map),
        }
    }

    pub fn set_password(&mut self, password: String) -> Result<(), Error> {
        let mut map = self.attributes.as_object_mut().unwrap().clone();
        let salt = SaltString::generate(&mut OsRng);
        let hash = self.firebase_scrypt.generate_base64_hash(&password, salt.as_str()).map_err(|e| format!("{e:?}"))?;
        map.insert("passwordHash".to_string(), Value::String(hash));
        map.insert("salt".to_string(), Value::String(salt.as_str().to_string()));
        self.attributes = Value::Object(map);
        Ok(())
    }

    pub fn verify(&self, password: &str) -> Result<bool, Error> {
        let salt = self.attributes.get("salt").unwrap().as_str().unwrap();
        let hash = self.attributes.get("passwordHash").unwrap().as_str().unwrap();
        Ok(self.firebase_scrypt.verify_password(password, salt, hash).map_err(|e| format!("{e:?}"))?)
    }
}

impl CredentialInterface for PasswordScryptCredential {
    fn identifier(&self) -> String {
        self.attributes
            .as_object()
            .unwrap()
            .get("identifier")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }

    fn set_identifier(&mut self, identifier: String) {
        let mut map = self.attributes.as_object_mut().unwrap().clone();
        map.insert("identifier".to_string(), Value::String(identifier));
        self.attributes = Value::Object(map);
    }
}
