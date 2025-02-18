use crate::context::BoscaContext;
use crate::graphql::content::storage::ObjectStorage;
use crate::models::content::collection::Collection;
use crate::models::content::metadata::Metadata;
use crate::models::content::search::SearchDocumentInput;
use crate::models::security::permission::PermissionAction;
use crate::models::workflow::storage_systems::{StorageSystem, StorageSystemType};
use async_graphql::Error;
use meilisearch_sdk::client::Client;
use meilisearch_sdk::errors::ErrorCode::IndexNotFound;
use serde_json::{Map, Value};
use uuid::Uuid;
use crate::models::profiles::profile_visibility::ProfileVisibility;

pub async fn storage_system_metadata_delete(
    storage: &ObjectStorage,
    metadata: &Metadata,
    storage_systems: &[StorageSystem],
    client: &Client,
) -> Result<(), Error> {
    if metadata.uploaded.is_some() {
        let path = storage.get_metadata_path(metadata, None).await?;
        storage.delete(&path).await?;
    }
    for storage_system in storage_systems.iter() {
        match storage_system.system_type {
            StorageSystemType::Search => {
                if let Some(configuration) = &storage_system.configuration {
                    let index_name = configuration
                        .get("indexName")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();
                    match client.get_index(&index_name).await {
                        Ok(index) => {
                            index.delete_document(&metadata.id.to_string()).await?;
                        }
                        Err(e) => {
                            if let meilisearch_sdk::errors::Error::Meilisearch(e) = e {
                                if e.error_code == IndexNotFound {
                                } else {
                                    return Err(Error::new(e.to_string()));
                                }
                            } else {
                                return Err(Error::new(e.to_string()));
                            }
                        }
                    }
                }
            }
            StorageSystemType::Vector => {
                // TODO
            }
            StorageSystemType::Supplementary => {
                // TODO
            }
        }
    }
    Ok(())
}

pub async fn storage_system_collection_delete(
    collection: &Collection,
    storage_system: &StorageSystem,
    client: &Client,
) -> Result<(), Error> {
    match storage_system.system_type {
        StorageSystemType::Search => {
            if let Some(configuration) = &storage_system.configuration {
                let index_name = configuration
                    .get("indexName")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();
                match client.get_index(&index_name).await {
                    Ok(index) => {
                        index.delete_document(&collection.id.to_string()).await?;
                    }
                    Err(e) => return Err(Error::new(e.to_string())),
                }
            }
        }
        StorageSystemType::Vector => {}
        StorageSystemType::Supplementary => {}
    }
    Ok(())
}

pub async fn index_documents(
    ctx: &BoscaContext,
    documents: &[SearchDocumentInput],
    storage_system: &StorageSystem,
) -> Result<(), Error> {
    let mut index_documents = Vec::new();
    for document in documents {
        if let Some(document) = create_search_document(ctx, document).await? {
            index_documents.push(document);
        }
    }
    if let Some(configuration) = &storage_system.configuration {
        let index_name = configuration
            .get("indexName")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let index = ctx.search.index(&index_name);
        index
            .add_documents(index_documents.as_slice(), Some("_id"))
            .await?;
    }
    Ok(())
}

// pub async fn index_documents_no_checks(ctx: &BoscaContext, documents: &[SearchDocumentInput], storage_system: &StorageSystem) -> Result<(), Error> {
//     let mut index_documents = Vec::new();
//     for document in documents {
//         if let Some(document) = create_search_document_no_checks(ctx, document).await? {
//             index_documents.push(document);
//         }
//     }
//     if let Some(configuration) = &storage_system.configuration {
//         let index_name = configuration.get("indexName").unwrap().as_str().unwrap().to_string();
//         let index = ctx.search.index(&index_name);
//         index.add_documents(index_documents.as_slice(), Some("_id")).await?;
//     }
//     Ok(())
// }

async fn create_search_document(
    ctx: &BoscaContext,
    document: &SearchDocumentInput,
) -> Result<Option<Map<String, Value>>, Error> {
    Ok(if let Some(metadata_id) = &document.metadata_id {
        let metadata_id_uuid = Uuid::parse_str(metadata_id.as_str())?;
        let metadata = ctx
            .check_metadata_action(&metadata_id_uuid, PermissionAction::Manage)
            .await?;
        let mut m = serde_json::Map::<String, Value>::new();
        m.insert("_id".to_owned(), Value::String(metadata_id.to_owned()));
        m.insert("_name".to_owned(), Value::String(metadata.name.to_owned()));
        m.insert(
            "_content".to_owned(),
            Value::String(document.content.to_owned()),
        );
        m.insert("_type".to_owned(), Value::String("metadata".to_owned()));
        if let Value::Object(attributes) = metadata.attributes {
            for attr in attributes.iter() {
                m.insert(attr.0.to_string().replace(".", "_"), attr.1.clone());
            }
        }
        Some(m)
    } else if let Some(collection_id) = &document.collection_id {
        let collection_id_uuid = Uuid::parse_str(collection_id.as_str())?;
        let collection = ctx
            .check_collection_action(&collection_id_uuid, PermissionAction::Manage)
            .await?;
        let mut m = serde_json::Map::<String, Value>::new();
        m.insert("_id".to_owned(), Value::String(collection_id.to_owned()));
        m.insert(
            "_name".to_owned(),
            Value::String(collection.name.to_owned()),
        );
        m.insert(
            "_content".to_owned(),
            Value::String(document.content.to_owned()),
        );
        m.insert("_type".to_owned(), Value::String("collection".to_owned()));
        if let Value::Object(attributes) = collection.attributes {
            for attr in attributes.iter() {
                m.insert(attr.0.to_string().replace(".", "_"), attr.1.clone());
            }
        }
        Some(m)
    } else if let Some(profile_id) = &document.profile_id {
        let profile_id_uuid = Uuid::parse_str(profile_id.as_str())?;
        let profile = ctx
            .check_profile_action(&profile_id_uuid, PermissionAction::Manage)
            .await?;
        let mut m = serde_json::Map::<String, Value>::new();
        m.insert("_id".to_owned(), Value::String(profile_id.to_owned()));
        m.insert("_name".to_owned(), Value::String(profile.name.to_owned()));
        m.insert(
            "_content".to_owned(),
            Value::String(document.content.to_owned()),
        );
        m.insert("_type".to_owned(), Value::String("profile".to_owned()));
        let mut attributes = ctx.profile.get_attributes(&profile_id_uuid).await?;
        attributes.sort_by(|a, b| {
            a.confidence.cmp(&b.confidence)
        });
        for attr in attributes.iter() {
            if attr.visibility != ProfileVisibility::Public {
                continue;
            }
            m.insert(attr.type_id.to_string().replace(".", "_"), attr.attributes.clone());
        }
        Some(m)
    } else {
        None
    })
}

// pub async fn create_search_document_no_checks(ctx: &BoscaContext, document: &SearchDocumentInput) -> Result<Option<Map<String, Value>>, Error> {
//     Ok(if let Some(metadata_id) = &document.metadata_id {
//         let metadata_id_uuid = Uuid::parse_str(metadata_id.as_str())?;
//         let metadata = ctx.content.get_metadata(&metadata_id_uuid).await?.unwrap();
//         let mut m = serde_json::Map::<String, Value>::new();
//         m.insert("_id".to_owned(), Value::String(metadata_id.to_owned()));
//         m.insert("_name".to_owned(), Value::String(metadata.name.to_owned()));
//         m.insert("_content".to_owned(), Value::String(document.content.to_owned()));
//         m.insert("_type".to_owned(), Value::String("metadata".to_owned()));
//         if let Value::Object(attributes) = metadata.attributes {
//             for attr in attributes.iter() {
//                 m.insert(attr.0.to_string().replace(".", "_"), attr.1.clone());
//             }
//         }
//         Some(m)
//     } else if let Some(collection_id) = &document.collection_id {
//         let collection_id_uuid = Uuid::parse_str(collection_id.as_str())?;
//         let collection = ctx.content.get_collection(&collection_id_uuid).await?.unwrap();
//         let mut m = serde_json::Map::<String, Value>::new();
//         m.insert("_id".to_owned(), Value::String(collection_id.to_owned()));
//         m.insert("_name".to_owned(), Value::String(collection.name.to_owned()));
//         m.insert("_content".to_owned(), Value::String(document.content.to_owned()));
//         m.insert("_type".to_owned(), Value::String("collection".to_owned()));
//         if let Value::Object(attributes) = collection.attributes {
//             for attr in attributes.iter() {
//                 m.insert(attr.0.to_string().replace(".", "_"), attr.1.clone());
//             }
//         }
//         Some(m)
//     } else {
//         None
//     })
// }
