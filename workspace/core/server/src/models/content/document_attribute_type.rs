use std::error::Error;
use async_graphql::Enum;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
// create type document_attribute_ui_type as enum ('text', 'image', 'profile', 'file');
#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq)]
pub enum DocumentAttributeType {
    String,
    Int,
    Float,
    Date,
    DateTime,
    Profile,
    Metadata,
    Collection,
}

impl<'a> FromSql<'a> for DocumentAttributeType {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> async_graphql::Result<DocumentAttributeType, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        Ok(match e.as_str() {
            "string" => DocumentAttributeType::String,
            "int" => DocumentAttributeType::Int,
            "float" => DocumentAttributeType::Float,
            "date" => DocumentAttributeType::Date,
            "datetime" => DocumentAttributeType::DateTime,
            "profile" => DocumentAttributeType::Profile,
            "metadata" => DocumentAttributeType::Metadata,
            "collection" => DocumentAttributeType::Collection,
            _ => DocumentAttributeType::String,
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "document_attribute_type"
    }
}

impl ToSql for DocumentAttributeType {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> async_graphql::Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            DocumentAttributeType::String => w.put_slice("string".as_ref()),
            DocumentAttributeType::Int => w.put_slice("int".as_ref()),
            DocumentAttributeType::Float => w.put_slice("float".as_ref()),
            DocumentAttributeType::Date => w.put_slice("date".as_ref()),
            DocumentAttributeType::DateTime => w.put_slice("datetime".as_ref()),
            DocumentAttributeType::Profile => w.put_slice("profile".as_ref()),
            DocumentAttributeType::Metadata => w.put_slice("metadata".as_ref()),
            DocumentAttributeType::Collection => w.put_slice("collection".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "document_attribute_type"
    }

    to_sql_checked!();
}