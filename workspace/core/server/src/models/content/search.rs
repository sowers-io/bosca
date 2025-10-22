use async_graphql::*;
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::profiles::profile::ProfileObject;
use crate::models::content::collection::Collection;
use crate::models::content::metadata::Metadata;
use crate::models::profiles::profile::Profile;

#[derive(SimpleObject)]
pub struct SearchResultObject {
    pub documents: Vec<SearchDocument>,
    pub facets: Vec<SearchResultFacet>,
    pub estimated_hits: i64
}

#[derive(SimpleObject)]
pub struct SearchResultFacet {
    pub field: String,
    pub value: String,
    pub count: i64,
}

pub struct SearchDocument {
    pub metadata: Option<Metadata>,
    pub collection: Option<Collection>,
    pub profile: Option<Profile>,
}

#[derive(InputObject)]
pub struct SearchQuery {
    pub storage_system_name: Option<String>,
    pub storage_system_id: Option<String>,
    pub query: String,
    pub filter: Option<String>,
    pub sort: Option<Vec<String>>,
    pub facets: Option<Vec<String>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub embedder: Option<String>,
}

#[Object(name = "SearchDocument")]
impl SearchDocument {

    async fn metadata(&self) -> Option<MetadataObject> {
        let metadata = self.metadata.as_ref()?;
        Some(MetadataObject::new(metadata.clone()))
    }

    async fn collection(&self) -> Option<CollectionObject> {
        let collection = self.collection.as_ref()?;
        Some(CollectionObject::new(collection.clone()))
    }

    async fn profile(&self) -> Option<ProfileObject> {
        let profile = self.profile.as_ref()?;
        Some(ProfileObject::new(profile.clone()))
    }
}