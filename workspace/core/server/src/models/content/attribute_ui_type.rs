use std::error::Error;
use async_graphql::Enum;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum AttributeUiType {
    Input,
    Textarea,
    Image,
    Profile,
    Collection,
    Metadata,
    File,
}

impl<'a> FromSql<'a> for AttributeUiType {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> async_graphql::Result<AttributeUiType, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        Ok(match e.as_str() {
            "input" => AttributeUiType::Input,
            "textarea" => AttributeUiType::Textarea,
            "image" => AttributeUiType::Image,
            "profile" => AttributeUiType::Profile,
            "collection" => AttributeUiType::Collection,
            "metadata" => AttributeUiType::Metadata,
            "file" => AttributeUiType::File,
            _ => AttributeUiType::Input,
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "attribute_ui_type"
    }
}

impl ToSql for AttributeUiType {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> async_graphql::Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            AttributeUiType::Input => w.put_slice("input".as_ref()),
            AttributeUiType::Textarea => w.put_slice("textarea".as_ref()),
            AttributeUiType::Image => w.put_slice("image".as_ref()),
            AttributeUiType::Profile => w.put_slice("profile".as_ref()),
            AttributeUiType::Collection => w.put_slice("collection".as_ref()),
            AttributeUiType::Metadata => w.put_slice("metadata".as_ref()),
            AttributeUiType::File => w.put_slice("file".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "attribute_ui_type"
    }

    to_sql_checked!();
}