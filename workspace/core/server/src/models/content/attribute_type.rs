use std::error::Error;
use async_graphql::Enum;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum AttributeType {
    #[default]
    String,
    Int,
    Float,
    Date,
    DateTime,
    Profile,
    Metadata,
    Collection,
}


impl<'a> FromSql<'a> for AttributeType {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> async_graphql::Result<AttributeType, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        Ok(match e.as_str() {
            "string" => AttributeType::String,
            "int" => AttributeType::Int,
            "float" => AttributeType::Float,
            "date" => AttributeType::Date,
            "datetime" => AttributeType::DateTime,
            "profile" => AttributeType::Profile,
            "metadata" => AttributeType::Metadata,
            "collection" => AttributeType::Collection,
            _ => AttributeType::String,
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "attribute_type"
    }
}

impl ToSql for AttributeType {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> async_graphql::Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            AttributeType::String => w.put_slice("string".as_ref()),
            AttributeType::Int => w.put_slice("int".as_ref()),
            AttributeType::Float => w.put_slice("float".as_ref()),
            AttributeType::Date => w.put_slice("date".as_ref()),
            AttributeType::DateTime => w.put_slice("datetime".as_ref()),
            AttributeType::Profile => w.put_slice("profile".as_ref()),
            AttributeType::Metadata => w.put_slice("metadata".as_ref()),
            AttributeType::Collection => w.put_slice("collection".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "attribute_type"
    }

    to_sql_checked!();
}