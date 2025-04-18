use async_graphql::Enum;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Enum, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Copy, Ord, PartialOrd)]
#[derive(Default)]
pub enum AttributeLocation {
    #[default]
    Item,
    Relationship,
}


impl<'a> FromSql<'a> for AttributeLocation {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> async_graphql::Result<AttributeLocation, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        Ok(match e.as_str() {
            "item" => AttributeLocation::Item,
            "relationship" => AttributeLocation::Relationship,
            _ => AttributeLocation::Item,
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "attribute_location"
    }
}

impl ToSql for AttributeLocation {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> async_graphql::Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            AttributeLocation::Item => w.put_slice("item".as_ref()),
            AttributeLocation::Relationship => w.put_slice("relationship".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "attribute_location"
    }

    to_sql_checked!();
}