use std::error::Error;
use async_graphql::Enum;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq)]
pub enum DocumentAttributeUiType {
    Input,
    Textarea,
    Image,
    Profile,
    Collection,
    Metadata,
    File,
}

impl<'a> FromSql<'a> for DocumentAttributeUiType {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> async_graphql::Result<DocumentAttributeUiType, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        Ok(match e.as_str() {
            "input" => DocumentAttributeUiType::Input,
            "textarea" => DocumentAttributeUiType::Textarea,
            "image" => DocumentAttributeUiType::Image,
            "profile" => DocumentAttributeUiType::Profile,
            "collection" => DocumentAttributeUiType::Collection,
            "metadata" => DocumentAttributeUiType::Metadata,
            "file" => DocumentAttributeUiType::File,
            _ => DocumentAttributeUiType::Input,
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "document_attribute_ui_type"
    }
}

impl ToSql for DocumentAttributeUiType {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> async_graphql::Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            DocumentAttributeUiType::Input => w.put_slice("input".as_ref()),
            DocumentAttributeUiType::Textarea => w.put_slice("textarea".as_ref()),
            DocumentAttributeUiType::Image => w.put_slice("image".as_ref()),
            DocumentAttributeUiType::Profile => w.put_slice("profile".as_ref()),
            DocumentAttributeUiType::Collection => w.put_slice("collection".as_ref()),
            DocumentAttributeUiType::Metadata => w.put_slice("metadata".as_ref()),
            DocumentAttributeUiType::File => w.put_slice("file".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "document_attribute_ui_type"
    }

    to_sql_checked!();
}