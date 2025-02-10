use std::error::Error;
use async_graphql::Enum;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq)]
pub enum DocumentMetadataAttributeType {
    String,
    Int,
    Float,
    Date,
    DateTime,
}

impl<'a> FromSql<'a> for DocumentMetadataAttributeType {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> async_graphql::Result<DocumentMetadataAttributeType, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        Ok(match e.as_str() {
            "string" => DocumentMetadataAttributeType::String,
            "int" => DocumentMetadataAttributeType::Int,
            "float" => DocumentMetadataAttributeType::Float,
            "date" => DocumentMetadataAttributeType::Date,
            "datetime" => DocumentMetadataAttributeType::Date,
            _ => DocumentMetadataAttributeType::String,
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "document_metadata_attribute_type"
    }
}

impl ToSql for DocumentMetadataAttributeType {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> async_graphql::Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            DocumentMetadataAttributeType::String => w.put_slice("string".as_ref()),
            DocumentMetadataAttributeType::Int => w.put_slice("int".as_ref()),
            DocumentMetadataAttributeType::Float => w.put_slice("float".as_ref()),
            DocumentMetadataAttributeType::Date => w.put_slice("date".as_ref()),
            DocumentMetadataAttributeType::DateTime => w.put_slice("datetime".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "document_metadata_attribute_type"
    }

    to_sql_checked!();
}