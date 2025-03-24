use async_graphql::{Enum, InputObject};
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use std::error::Error;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum PermissionAction {
    View,
    Edit,
    Delete,
    Manage,
    List,
    Impersonate,
    Execute
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    pub entity_id: Uuid,
    pub group_id: Uuid,
    pub action: PermissionAction,
}

#[derive(InputObject, Clone)]
pub struct PermissionInput {
    pub entity_id: String,
    pub group_id: String,
    pub action: PermissionAction,
}

impl From<PermissionInput> for Permission {
    fn from(value: PermissionInput) -> Self {
        Permission {
            entity_id: Uuid::parse_str(value.entity_id.as_str()).unwrap(),
            group_id: Uuid::parse_str(value.group_id.as_str()).unwrap(),
            action: value.action,
        }
    }
}

impl From<&Row> for Permission {
    fn from(row: &Row) -> Self {
        Self {
            entity_id: row.get("entity_id"),
            group_id: row.get("group_id"),
            action: row.get("action"),
        }
    }
}

impl<'a> FromSql<'a> for PermissionAction {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> async_graphql::Result<PermissionAction, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        Ok(match e.as_str() {
            "view" => PermissionAction::View,
            "edit" => PermissionAction::Edit,
            "delete" => PermissionAction::Delete,
            "manage" => PermissionAction::Manage,
            "list" => PermissionAction::List,
            "impersonate" => PermissionAction::Impersonate,
            "execute" => PermissionAction::Execute,
            _ => PermissionAction::View,
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "permission_action"
    }
}

impl ToSql for PermissionAction {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> async_graphql::Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            PermissionAction::View => w.put_slice("view".as_ref()),
            PermissionAction::Edit => w.put_slice("edit".as_ref()),
            PermissionAction::Delete => w.put_slice("delete".as_ref()),
            PermissionAction::Manage => w.put_slice("manage".as_ref()),
            PermissionAction::List => w.put_slice("list".as_ref()),
            PermissionAction::Impersonate => w.put_slice("impersonate".as_ref()),
            PermissionAction::Execute => w.put_slice("execute".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "permission_action"
    }

    to_sql_checked!();
}
