use crate::models::security::password::encrypt;
use async_graphql::{Enum, Error};
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde_json::{Map, Value};

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq)]
pub enum CredentialType {
    Password,
    Oauth2,
}

pub trait CredentialInterface {
    fn identifier(&self) -> String;
    fn set_identifier(&mut self, identifier: String);
}

pub enum Credential {
    Password(PasswordCredential)
}

impl Credential {
    pub fn get_type(&self) -> CredentialType {
        match self {
            Credential::Password(c) => c.credential_type
        }
    }

    pub fn identifier(&self) -> String {
        match self {
            Credential::Password(c) => c.identifier()
        }
    }

    pub fn get_attributes(&self) -> Value {
        match self {
            Credential::Password(c) => c.attributes.clone()
        }
    }

    pub fn set_identifier(&mut self, identifier: String) {
        match self {
            Credential::Password(c) => c.set_identifier(identifier)
        }
    }

    pub fn set_password(&mut self, password: String) -> Result<(), Error> {
        match self {
            Credential::Password(c) => c.set_password(password)
        }
    }
}

pub struct PasswordCredential {
    credential_type: CredentialType,
    attributes: Value,
}

impl PasswordCredential {
    pub fn new(identifier: String, password: String) -> Result<Self, Error> {
        let mut map = Map::<String, Value>::new();
        map.insert("identifier".to_string(), Value::String(identifier.to_lowercase()));
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

impl<'a> FromSql<'a> for CredentialType {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> async_graphql::Result<CredentialType, Box<dyn std::error::Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        match e.as_str() {
            "password" => Ok(CredentialType::Password),
            "oauth2" => Ok(CredentialType::Oauth2),
            _ => Ok(CredentialType::Password),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "principal_credential_type"
    }
}

impl ToSql for CredentialType {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> async_graphql::Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        match *self {
            CredentialType::Password => w.put_slice("password".as_ref()),
            CredentialType::Oauth2 => w.put_slice("oauth2".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "principal_credential_type"
    }

    to_sql_checked!();
}
