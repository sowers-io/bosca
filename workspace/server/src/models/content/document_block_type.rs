use std::error::Error;
use async_graphql::Enum;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq)]
pub enum DocumentBlockType {
    Text,
    RichText,
    Video,
    Audio,
    Image,
    Supplementary,
}

impl<'a> FromSql<'a> for DocumentBlockType {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> async_graphql::Result<DocumentBlockType, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        Ok(match e.as_str() {
            "text" => DocumentBlockType::Text,
            "richtext" => DocumentBlockType::RichText,
            "video" => DocumentBlockType::Video,
            "audio" => DocumentBlockType::Audio,
            "image" => DocumentBlockType::Image,
            "supplementary" => DocumentBlockType::Supplementary,
            _ => DocumentBlockType::Text,
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "document_block_type"
    }
}

impl ToSql for DocumentBlockType {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> async_graphql::Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            DocumentBlockType::Text => w.put_slice("text".as_ref()),
            DocumentBlockType::RichText => w.put_slice("richtext".as_ref()),
            DocumentBlockType::Video => w.put_slice("video".as_ref()),
            DocumentBlockType::Audio => w.put_slice("audio".as_ref()),
            DocumentBlockType::Image => w.put_slice("image".as_ref()),
            DocumentBlockType::Supplementary => w.put_slice("supplementary".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "document_block_type"
    }

    to_sql_checked!();
}