use async_graphql::Enum;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum GuideType {
    Linear,
    LinearProgress,
    Calendar,
    CalendarProgress,
}

impl<'a> FromSql<'a> for GuideType {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> async_graphql::Result<GuideType, Box<dyn Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        Ok(match e.as_str() {
            "linear" => GuideType::Linear,
            "linear_progress" => GuideType::LinearProgress,
            "calendar" => GuideType::Calendar,
            "calendar_progress" => GuideType::CalendarProgress,
            _ => GuideType::Linear,
        })
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "guide_type"
    }
}

impl ToSql for GuideType {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> async_graphql::Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            GuideType::Linear => w.put_slice("linear".as_ref()),
            GuideType::LinearProgress => w.put_slice("linear_progress".as_ref()),
            GuideType::Calendar => w.put_slice("calendar".as_ref()),
            GuideType::CalendarProgress => w.put_slice("calendar_progress".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "guide_type"
    }

    to_sql_checked!();
}
