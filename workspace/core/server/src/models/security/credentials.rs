use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde_json::{Map, Value};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CredentialType {
    Password,
    Oauth2,
}

pub trait Credential {
    fn get_type(&self) -> CredentialType;
    fn get_attributes(&self) -> Value;
}

pub struct PasswordCredential {
    credential_type: CredentialType,
    attributes: Value,
}

impl PasswordCredential {
    pub fn new(identifier: String, password: String) -> Self {
        let mut map = Map::<String, Value>::new();
        map.insert("identifier".to_string(), Value::String(identifier));
        map.insert("password".to_string(), Value::String(password));
        Self {
            credential_type: CredentialType::Password,
            attributes: Value::Object(map),
        }
    }

    // fn get_identifier(&self) -> String {
    //     self.attributes.as_object().unwrap().get("identifier").unwrap().as_str().unwrap().to_string()
    // }
    //
    // fn get_password(&self) -> String {
    //     self.attributes.as_object().unwrap().get("password").unwrap().as_str().unwrap().to_string()
    // }
}

impl Credential for PasswordCredential {
    fn get_type(&self) -> CredentialType {
        self.credential_type
    }
    fn get_attributes(&self) -> Value {
        self.attributes.clone()
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
