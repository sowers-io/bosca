use crate::models::security::credentials_oauth2::Oauth2Credential;
use crate::models::security::credentials_password::PasswordCredential;
use async_graphql::{Enum, Error};
use bytes::{BufMut, BytesMut};
use oauth2::TokenResponse;
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde_json::Value;
use std::fmt::Debug;
use crate::models::security::credentials_scrypt::PasswordScryptCredential;

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq)]
pub enum CredentialType {
    Password,
    PasswordScrypt,
    Oauth2,
}

pub trait CredentialInterface {
    fn identifier(&self) -> String;
    fn set_identifier(&mut self, identifier: String);
}

pub enum Credential {
    Password(PasswordCredential),
    PasswordScrypt(PasswordScryptCredential),
    Oauth2(Oauth2Credential),
}

impl Credential {
    pub fn get_type(&self) -> CredentialType {
        match self {
            Credential::Password(c) => c.credential_type,
            Credential::PasswordScrypt(c) => c.credential_type,
            Credential::Oauth2(c) => c.credential_type,
        }
    }

    pub fn identifier(&self) -> String {
        match self {
            Credential::Password(c) => c.identifier(),
            Credential::PasswordScrypt(c) => c.identifier(),
            Credential::Oauth2(c) => c.identifier(),
        }
    }

    pub fn identifier_type(&self) -> Option<String> {
        match self {
            Credential::Password(_) => None,
            Credential::PasswordScrypt(c) => c.attributes["localId"].as_str().map(|s| s.to_string()),
            Credential::Oauth2(c) => Some(c.attributes["type"].as_str().unwrap().to_string()),
        }
    }

    pub fn get_attributes(&self) -> Value {
        match self {
            Credential::Password(c) => c.attributes.clone(),
            Credential::PasswordScrypt(c) => c.attributes.clone(),
            Credential::Oauth2(c) => c.attributes.clone(),
        }
    }

    pub fn set_identifier(&mut self, identifier: String) {
        match self {
            Credential::Password(c) => c.set_identifier(identifier),
            Credential::PasswordScrypt(c) => c.set_identifier(identifier),
            Credential::Oauth2(c) => c.set_identifier(identifier),
        }
    }

    pub fn set_password(&mut self, password: String) -> Result<(), Error> {
        match self {
            Credential::Password(c) => c.set_password(password),
            Credential::PasswordScrypt(c) => c.set_password(password),
            Credential::Oauth2(_) => Err(Error::new("Cannot set password on Oauth2 credential")),
        }
    }

    pub fn set_tokens(&mut self, password: impl TokenResponse) -> Result<(), Error> {
        match self {
            Credential::Password(_) => Err(Error::new("Cannot set tokens on Password credential")),
            Credential::PasswordScrypt(_) => Err(Error::new("Cannot set tokens on PasswordScrypt credential")),
            Credential::Oauth2(c) => c.set_tokens(password),
        }
    }

    pub fn verify(&self, password: &str) -> Result<bool, Error> {
        match self {
            Credential::Password(c) => c.verify(password),
            Credential::PasswordScrypt(c) => c.verify(password),
            Credential::Oauth2(_) => Ok(false)
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
            "password_scrypt" => Ok(CredentialType::PasswordScrypt),
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
            CredentialType::PasswordScrypt => w.put_slice("password_scrypt".as_ref()),
            CredentialType::Oauth2 => w.put_slice("oauth2".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "principal_credential_type"
    }

    to_sql_checked!();
}
