use std::error::Error;
use async_graphql::Enum;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum CommentStatus {
    Pending,
    Blocked,
    PendingApproval,
    Approved,
}

impl<'a> FromSql<'a> for CommentStatus {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> async_graphql::Result<CommentStatus, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        Ok(match e.as_str() {
            "pending" => CommentStatus::Pending,
            "blocked" => CommentStatus::Blocked,
            "pending_approval" => CommentStatus::PendingApproval,
            "approved" => CommentStatus::Approved,
            _ => CommentStatus::Pending,
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "comment_status"
    }
}

impl ToSql for CommentStatus {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> async_graphql::Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            CommentStatus::Pending => w.put_slice("pending".as_ref()),
            CommentStatus::Blocked => w.put_slice("blocked".as_ref()),
            CommentStatus::PendingApproval => w.put_slice("pending_approval".as_ref()),
            CommentStatus::Approved => w.put_slice("approved".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "comment_status"
    }

    to_sql_checked!();
}