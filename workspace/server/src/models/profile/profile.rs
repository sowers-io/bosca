use async_graphql::{Enum, InputObject};
use std::error::Error;
use bytes::{BufMut, BytesMut};
use chrono::{DateTime, Utc};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ProfileVisibility {
    System,
    User,
    Friends,
    FriendsOfFriends,
    Public,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    pub id: Uuid,
    pub principal: Uuid,
    pub name: String,
    pub visibility: ProfileVisibility
}

#[derive(InputObject, Debug, Clone, PartialEq, Eq)]
pub struct ProfileInput {
    pub name: String,
    pub visibility: ProfileVisibility,
    pub attributes: Vec<ProfileAttributeInput>,
}

#[derive(InputObject, Debug, Clone, PartialEq, Eq)]
pub struct ProfileAttributeInput {
    pub id: Option<String>,
    pub type_id: String,
    pub visibility: ProfileVisibility,
    pub confidence: i32,
    pub priority: i32,
    pub source: String,
    pub attributes: Value,
    pub expiration: Option<DateTime<Utc>>
}

impl From<&Row> for Profile {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            principal: row.get("principal"),
            name: row.get("name"),
            visibility: row.get("visibility")
        }
    }
}

impl<'a> FromSql<'a> for ProfileVisibility {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> async_graphql::Result<ProfileVisibility, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        Ok(match e.as_str() {
            "system" => ProfileVisibility::System,
            "user" => ProfileVisibility::User,
            "friends" => ProfileVisibility::Friends,
            "friends_of_friends" => ProfileVisibility::FriendsOfFriends,
            "public" => ProfileVisibility::Public,
            _ => ProfileVisibility::System,
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "profile_visibility"
    }
}

impl ToSql for ProfileVisibility {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> async_graphql::Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            ProfileVisibility::System => w.put_slice("system".as_ref()),
            ProfileVisibility::User => w.put_slice("user".as_ref()),
            ProfileVisibility::Friends => w.put_slice("friends".as_ref()),
            ProfileVisibility::FriendsOfFriends => w.put_slice("friends_of_friends".as_ref()),
            ProfileVisibility::Public => w.put_slice("public".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "profile_visibility"
    }

    to_sql_checked!();
}