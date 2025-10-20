use crate::models::content::source::Source;
use async_graphql::{Context, Error, Object};
use serde_json::Value;

pub struct SourceObject {
    source: Source,
}

impl SourceObject {
    pub fn new(source: Source) -> Self {
        Self { source }
    }
}

#[Object(name = "Source")]
impl SourceObject {
    async fn id(&self) -> String {
        self.source.id.to_string()
    }

    async fn name(&self) -> &String {
        &self.source.name
    }

    async fn description(&self) -> &String {
        &self.source.description
    }

    async fn configuration(&self, ctx: &Context<'_>) -> Result<Value, Error> {
        let ctx = ctx.data::<crate::context::BoscaContext>()?;
        if ctx.check_has_service_account().await.is_ok() {
            Ok(self.source.configuration.clone())
        } else {
            Ok(Value::Null)
        }
    }
}

impl From<Source> for SourceObject {
    fn from(source: Source) -> Self {
        Self::new(source)
    }
}
