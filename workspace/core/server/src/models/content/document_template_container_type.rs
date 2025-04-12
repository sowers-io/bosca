use async_graphql::Enum;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Enum, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Copy, Ord, PartialOrd)]
pub enum DocumentTemplateContainerType {
    Standard,
    Bible,
}
impl<'a> FromSql<'a> for DocumentTemplateContainerType {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> async_graphql::Result<DocumentTemplateContainerType, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        Ok(match e.as_str() {
            "standard" => DocumentTemplateContainerType::Standard,
            "bible" => DocumentTemplateContainerType::Bible,
            _ => DocumentTemplateContainerType::Standard,
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "document_template_container_type"
    }
}

impl ToSql for DocumentTemplateContainerType {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> async_graphql::Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            DocumentTemplateContainerType::Standard => w.put_slice("standard".as_ref()),
            DocumentTemplateContainerType::Bible => w.put_slice("bible".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "document_template_container_type"
    }

    to_sql_checked!();
}