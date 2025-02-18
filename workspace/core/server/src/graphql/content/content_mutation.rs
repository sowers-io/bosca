use crate::graphql::content::collection_mutation::CollectionMutationObject;
use crate::graphql::content::metadata_mutation::MetadataMutationObject;
use async_graphql::{Context, Error, Object};
use log::error;
use crate::context::BoscaContext;
use crate::graphql::content::category_mutation::CategoryMutationObject;
use crate::models::content::search::SearchDocumentInput;
use crate::util::storage::index_documents;

pub struct ContentMutationObject {}

#[Object(name = "ContentMutation")]
impl ContentMutationObject {
    async fn category(&self) -> CategoryMutationObject { CategoryMutationObject{} }
    async fn collection(&self) -> CollectionMutationObject {
        CollectionMutationObject {}
    }
    async fn metadata(&self) -> MetadataMutationObject {
        MetadataMutationObject {}
    }

    async fn reindex(&self, ctx: &Context<'_>) -> async_graphql::Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal.has_group(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        const LIMIT: i64 = 100;
        let mut offset = 0;
        let storage_system = ctx.workflow.get_default_search_storage_system().await?;
        let mut search_documents = Vec::new();
        loop {
            let items = ctx.content.collections.get_all(offset, LIMIT).await?;
            if items.is_empty() {
                break;
            }
            offset += LIMIT;
            for item in items {
                search_documents.push(SearchDocumentInput {
                    collection_id: Some(item.id.to_string()),
                    metadata_id: None,
                    profile_id: None,
                    content: "".to_owned(),
                });
            }
            if let Some(storage_system) = &storage_system {
                index_documents(ctx, &search_documents, storage_system).await?;
            } else {
                error!("error, failed to index, no storage system")
            }
            search_documents.clear();
        }
        offset = 0;
        loop {
            let items = ctx.content.metadata.get_all(offset, LIMIT).await?;
            if items.is_empty() {
                break;
            }
            offset += LIMIT;
            for item in items {
                search_documents.push(SearchDocumentInput {
                    collection_id: None,
                    metadata_id: Some(item.id.to_string()),
                    profile_id: None,
                    content: "".to_owned(),
                });
            }
            if let Some(storage_system) = &storage_system {
                index_documents(ctx, &search_documents, storage_system).await?;
            } else {
                error!("error, failed to index, no storage system")
            }
            search_documents.clear();
        }
        offset = 0;
        loop {
            let items = ctx.profile.get_all(offset, LIMIT).await?;
            if items.is_empty() {
                break;
            }
            offset += LIMIT;
            for item in items {
                search_documents.push(SearchDocumentInput {
                    collection_id: None,
                    metadata_id: None,
                    profile_id: Some(item.id.to_string()),
                    content: "".to_owned(),
                });
            }
            if let Some(storage_system) = &storage_system {
                index_documents(ctx, &search_documents, storage_system).await?;
            } else {
                error!("error, failed to index, no storage system")
            }
            search_documents.clear();
        }
        Ok(true)
    }
}
