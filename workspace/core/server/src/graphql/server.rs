use async_graphql::Object;
use chrono::{DateTime, Utc};

pub struct ServerObject {
}

#[Object(name = "Server")]
impl ServerObject {

    async fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}