use crate::models::security::credentials::{CredentialInterface, CredentialType};
use crate::models::security::password::encrypt;
use async_graphql::Error;
use serde_json::{Map, Value};
use std::fmt::{Debug, Formatter};

pub struct PasswordCredential {
    pub credential_type: CredentialType,
    pub attributes: Value,
}

impl Debug for PasswordCredential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PasswordCredential").finish()
    }
}

impl PasswordCredential {
    pub fn new(identifier: String, password: String) -> Result<Self, Error> {
        if password.trim().is_empty() {
            return Err(Error::from("Password is Required"));
        }
        let mut map = Map::<String, Value>::new();
        map.insert(
            "identifier".to_string(),
            Value::String(identifier.to_lowercase()),
        );
        map.insert("password".to_string(), Value::String(encrypt(password)?));
        Ok(Self {
            credential_type: CredentialType::Password,
            attributes: Value::Object(map),
        })
    }

    pub fn new_from_attributes(attributes: Value) -> Self {
        Self {
            credential_type: CredentialType::Password,
            attributes,
        }
    }

    pub fn set_password(&mut self, password: String) -> Result<(), Error> {
        let mut map = self.attributes.as_object_mut().unwrap().clone();
        map.insert("password".to_string(), Value::String(encrypt(password)?));
        self.attributes = Value::Object(map);
        Ok(())
    }
}

impl CredentialInterface for PasswordCredential {
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

impl From<Value> for PasswordCredential {
    fn from(value: Value) -> Self {
        Self {
            credential_type: CredentialType::Password,
            attributes: value,
        }
    }
}
