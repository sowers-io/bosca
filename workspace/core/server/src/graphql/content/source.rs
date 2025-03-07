use crate::models::content::source::Source;
use async_graphql::Object;
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

    async fn configuration(&self) -> &Value {
        &self.source.configuration
    }
}

impl From<Source> for SourceObject {
    fn from(source: Source) -> Self {
        Self::new(source)
    }
}
