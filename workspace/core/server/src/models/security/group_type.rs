use std::error::Error;
use async_graphql::Enum;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum GroupType {
    System,
    Principal
}

impl<'a> FromSql<'a> for GroupType {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> async_graphql::Result<GroupType, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        Ok(match e.as_str() {
            "system" => GroupType::System,
            "principal" => GroupType::Principal,
            _ => GroupType::System,
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "group_type"
    }
}

impl ToSql for GroupType {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> async_graphql::Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            GroupType::System => w.put_slice("system".as_ref()),
            GroupType::Principal => w.put_slice("principal".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "group_type"
    }

    to_sql_checked!();
}