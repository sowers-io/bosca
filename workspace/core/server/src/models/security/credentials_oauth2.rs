use crate::models::security::credentials::{CredentialInterface, CredentialType};
use crate::models::security::password::encrypt;
use crate::security::account::Account;
use async_graphql::Error;
use oauth2::TokenResponse;
use serde_json::{Map, Value};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Oauth2Credential {
    pub credential_type: CredentialType,
    pub attributes: Value,
}

impl Debug for Oauth2Credential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Oauth2Credential").finish()
    }
}

impl Oauth2Credential {
    pub fn new(account: &Account, tokens: Option<&impl TokenResponse>) -> Result<Self, Error> {
        let Some(account_identifier) = account.id() else {
            return Err(Error::new("Account identifier is required"));
        };
        let mut map = Map::<String, Value>::new();
        map.insert(
            "identifier".to_string(),
            Value::String(account_identifier.to_lowercase()),
        );
        map.insert(
            "type".to_string(),
            Value::String(account.oauth2_type().to_lowercase()),
        );
        if let Some(tokens) = tokens {
            let tokens = serde_json::to_string(&tokens)?;
            map.insert("tokens".to_string(), Value::String(encrypt(tokens)?));
        }
        Ok(Self {
            credential_type: CredentialType::Oauth2,
            attributes: Value::Object(map),
        })
    }

    pub fn new_from_attributes(attributes: Value) -> Self {
        Self {
            credential_type: CredentialType::Oauth2,
            attributes,
        }
    }

    pub fn set_attribute(&mut self, key: &str, value: Value) {
        let mut map = self.attributes.as_object_mut().unwrap().clone();
        map.insert(key.to_string(), value);
        self.attributes = Value::Object(map);
    }

    pub fn set_tokens(&mut self, tokens: impl TokenResponse) -> Result<(), Error> {
        let tokens = serde_json::to_string(&tokens)?;
        let mut map = self.attributes.as_object_mut().unwrap().clone();
        map.insert("tokens".to_string(), Value::String(encrypt(tokens)?));
        self.attributes = Value::Object(map);
        Ok(())
    }
}

impl CredentialInterface for Oauth2Credential {
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

impl From<Value> for Oauth2Credential {
    fn from(value: Value) -> Self {
        Self {
            credential_type: CredentialType::Oauth2,
            attributes: value,
        }
    }
}
