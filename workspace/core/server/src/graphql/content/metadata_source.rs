use async_graphql::Object;
use crate::models::content::metadata::Metadata;

pub struct MetadataSourceObject {
    pub metadata: Metadata,
}

#[Object(name = "MetadataSource")]
impl MetadataSourceObject {
    async fn id(&self) -> Option<String> {
        self.metadata.source_id.map(|s| s.to_string())
    }

    async fn identifier(&self) -> &Option<String> {
        &self.metadata.source_identifier
    }

    async fn source_url(&self) -> &Option<String> {
        &self.metadata.source_url
    }
}
