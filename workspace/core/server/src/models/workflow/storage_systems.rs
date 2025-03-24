use crate::workflow::yaml::into;
use async_graphql::*;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;
use yaml_rust2::Yaml;
use crate::models::workflow::storage_system_models::StorageSystemModelInput;

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum StorageSystemType {
    Search,
    Vector,
    Supplementary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSystem {
    pub id: Uuid,
    pub system_type: StorageSystemType,
    pub name: String,
    pub description: String,
    pub configuration: Option<Value>,
}

#[derive(InputObject)]
pub struct StorageSystemInput {
    #[graphql(name = "type")]
    pub system_type: StorageSystemType,
    pub name: String,
    pub description: String,
    pub configuration: Option<Value>,
    pub models: Vec<StorageSystemModelInput>,
}

impl From<&Row> for StorageSystem {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            system_type: row.get("type"),
            description: row.get("description"),
            configuration: row.get("configuration"),
        }
    }
}

impl<'a> FromSql<'a> for StorageSystemType {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> Result<StorageSystemType, Box<dyn std::error::Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        match e.as_str() {
            "search" => Ok(StorageSystemType::Search),
            "vector" => Ok(StorageSystemType::Vector),
            "supplementary" => Ok(StorageSystemType::Supplementary),
            _ => Ok(StorageSystemType::Supplementary),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "storage_system_type"
    }
}

impl ToSql for StorageSystemType {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        match *self {
            StorageSystemType::Search => w.put_slice("search".as_ref()),
            StorageSystemType::Vector => w.put_slice("vector".as_ref()),
            StorageSystemType::Supplementary => w.put_slice("supplementary".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "storage_system_type"
    }

    to_sql_checked!();
}

impl From<&Yaml> for StorageSystem {
    fn from(yaml: &Yaml) -> Self {
        Self {
            id: if yaml["id"].is_null() || yaml["id"].is_badvalue() {
                Uuid::nil()
            } else {
                Uuid::parse_str(yaml["id"].as_str().unwrap()).unwrap()
            },
            name: yaml["name"].as_str().unwrap().to_string(),
            system_type: StorageSystemType::from_sql(
                &Type::VARCHAR,
                yaml["type"].as_str().unwrap().as_ref(),
            )
            .unwrap(),
            description: yaml["description"].as_str().unwrap().to_string(),
            configuration: Some(into(&yaml["configuration"])),
        }
    }
}

impl From<&Yaml> for StorageSystemInput {
    fn from(yaml: &Yaml) -> Self {
        Self {
            name: yaml["name"].as_str().unwrap_or("").to_string(),
            system_type: StorageSystemType::from_sql(
                &Type::VARCHAR,
                yaml["type"].as_str().unwrap().as_ref(),
            )
            .unwrap(),
            description: yaml["description"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            configuration: Some(into(&yaml["configuration"])),
            models: vec![],
        }
    }
}
