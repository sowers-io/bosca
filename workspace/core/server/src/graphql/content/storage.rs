use crate::datastores::security::SecurityDataStore;
use crate::models::content::collection::Collection;
use crate::models::content::metadata::Metadata;
use crate::models::content::signed_url::{SignedUrl, SignedUrlHeader};
use crate::models::security::principal::Principal;
use bytes::Bytes;
use futures_util::stream::BoxStream;
use object_store::aws::AmazonS3;
use object_store::local::LocalFileSystem;
use object_store::path::Path;
use object_store::{Error, MultipartUpload, ObjectStore, PutPayload};
use std::env;
use std::str::from_utf8;
use std::string::ToString;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct ObjectStorage {
    interface: Arc<ObjectStorageInterface>,
}

pub enum ObjectStorageInterface {
    FileSystem(Arc<LocalFileSystem>),
    #[allow(dead_code)]
    S3(Arc<AmazonS3>),
}

impl ObjectStorage {
    pub fn new(interface: ObjectStorageInterface) -> Self {
        Self {
            interface: Arc::new(interface),
        }
    }

    pub async fn get_metadata_path(
        &self,
        metadata: &Metadata,
        supplementary_id: Option<Uuid>,
    ) -> Result<Path, object_store::path::Error> {
        if let Some(supplementary_id) = supplementary_id {
            Path::parse(format!(
                "metadata/{}/{}/supplementary/{}",
                metadata.id, metadata.version, supplementary_id,
            ))
        } else {
            Path::parse(format!(
                "metadata/{}/{}/content",
                metadata.id, metadata.version
            ))
        }
    }

    pub async fn get_collection_path(
        &self,
        collection: &Collection,
        supplementary_id: Option<Uuid>,
    ) -> Result<Path, object_store::path::Error> {
        if let Some(supplementary_id) = supplementary_id {
            Path::parse(format!(
                "collection/{}/supplementary/{}",
                collection.id, supplementary_id,
            ))
        } else {
            Path::parse(format!("collection/{}/content", collection.id))
        }
    }

    pub async fn get(&self, location: &Path) -> Result<String, Error> {
        let result = match &self.interface.as_ref() {
            ObjectStorageInterface::FileSystem(fs) => fs.get(location),
            ObjectStorageInterface::S3(s3) => s3.get(location),
        }
        .await?;
        let bytes = result.bytes().await?;
        Ok(from_utf8(&bytes).unwrap().to_string())
    }

    pub async fn get_buffer(
        &self,
        location: &Path,
    ) -> Result<BoxStream<'static, object_store::Result<Bytes>>, Error> {
        let result = match &self.interface.as_ref() {
            ObjectStorageInterface::FileSystem(fs) => fs.get(location),
            ObjectStorageInterface::S3(s3) => s3.get(location),
        }
        .await?;
        let stream = result.into_stream();
        Ok(stream)
    }

    pub async fn delete(&self, location: &Path) -> Result<(), Error> {
        match &self.interface.as_ref() {
            ObjectStorageInterface::FileSystem(fs) => fs.delete(location),
            ObjectStorageInterface::S3(s3) => s3.delete(location),
        }
        .await?;
        Ok(())
    }

    pub async fn put_multipart(&self, location: &Path) -> Result<Box<dyn MultipartUpload>, Error> {
        match &self.interface.as_ref() {
            ObjectStorageInterface::FileSystem(fs) => fs.put_multipart(location),
            ObjectStorageInterface::S3(s3) => s3.put_multipart(location),
        }
        .await
    }

    pub async fn put(&self, location: &Path, bytes: Bytes) -> Result<(), Error> {
        let payload = PutPayload::from(bytes);
        match &self.interface.as_ref() {
            ObjectStorageInterface::FileSystem(fs) => fs.put(location, payload),
            ObjectStorageInterface::S3(s3) => s3.put(location, payload),
        }
        .await?;
        Ok(())
    }

    pub async fn get_metadata_upload_signed_url(
        &self,
        datastore: &SecurityDataStore,
        principal: &Principal,
        metadata: &Metadata,
        supplementary: Option<Uuid>,
    ) -> Result<SignedUrl, Error> {
        // match &self.interface.as_ref() {
        //     ObjectStorageInterface::FileSystem(_) => {
        //         let url = if supplementary.is_none() {
        //             format!(
        //                 "{}/files/upload?id={}",
        //                 env::var("BOSCA_URL_PREFIX").unwrap_or("".to_string()),
        //                 metadata.id
        //             )
        //         } else {
        //             format!(
        //                 "{}/files/upload?id={}&supplementary_id={}",
        //                 env::var("BOSCA_URL_PREFIX").unwrap_or("".to_string()),
        //                 metadata.id,
        //                 supplementary.unwrap()
        //             )
        //         };
        //         let token = match datasource.new_token(principal) {
        //             Ok(token) => token.token,
        //             Err(e) => {
        //                 return Err(Error::PermissionDenied {
        //                     path: url,
        //                     source: Box::new(e),
        //                 })
        //             }
        //         };
        //         Ok(SignedUrl {
        //             url,
        //             headers: vec![SignedUrlHeader {
        //                 name: "Authorization".to_string(),
        //                 value: format!("Bearer {}", token),
        //             }],
        //         })
        //     }
        //     ObjectStorageInterface::S3(fs) => {
        //         let path = self.get_metadata_path(metadata, supplementary).await?;
        //         let url = fs
        //             .signed_url(Method::POST, &path, Duration::from_secs(500))
        //             .await?;
        //         Ok(SignedUrl {
        //             url: url.to_string(),
        //             headers: vec![],
        //         })
        //     }
        // }
        let url = if supplementary.is_none() {
            format!(
                "{}/files/metadata/upload?id={}",
                env::var("BOSCA_UPLOAD_URL_PREFIX").unwrap_or(env::var("BOSCA_URL_PREFIX").unwrap_or("".to_string())),
                metadata.id
            )
        } else {
            format!(
                "{}/files/metadata/upload?id={}&supplementary_id={}",
                env::var("BOSCA_UPLOAD_URL_PREFIX").unwrap_or(env::var("BOSCA_URL_PREFIX").unwrap_or("".to_string())),
                metadata.id,
                supplementary.unwrap()
            )
        };
        let token = match datastore.new_token(principal) {
            Ok(token) => token.token,
            Err(e) => {
                return Err(Error::PermissionDenied {
                    path: url,
                    source: Box::new(e),
                })
            }
        };
        Ok(SignedUrl {
            url,
            headers: vec![SignedUrlHeader {
                name: "Authorization".to_string(),
                value: format!("Bearer {token}"),
            }],
        })
    }

    pub async fn get_metadata_download_signed_url(
        &self,
        datasource: &SecurityDataStore,
        principal: &Principal,
        metadata: &Metadata,
        supplementary: Option<Uuid>,
    ) -> Result<SignedUrl, Error> {
        // match &self.interface.as_ref() {
        //     ObjectStorageInterface::FileSystem(_) => {
        //         let url = if supplementary.is_none() {
        //             format!(
        //                 "{}/files/download?id={}",
        //                 env::var("BOSCA_URL_PREFIX").unwrap_or("".to_string()),
        //                 metadata.id
        //             )
        //         } else {
        //             format!(
        //                 "{}/files/download?id={}&supplementary_id={}",
        //                 env::var("BOSCA_URL_PREFIX").unwrap_or("".to_string()),
        //                 metadata.id,
        //                 supplementary.unwrap()
        //             )
        //         };
        //         let token = match datasource.new_token(principal) {
        //             Ok(token) => token.token,
        //             Err(e) => {
        //                 return Err(Error::PermissionDenied {
        //                     path: url,
        //                     source: Box::new(e),
        //                 })
        //             }
        //         };
        //         Ok(SignedUrl {
        //             url,
        //             headers: vec![SignedUrlHeader {
        //                 name: "Authorization".to_string(),
        //                 value: format!("Bearer {}", token),
        //             }],
        //         })
        //     }
        //     ObjectStorageInterface::S3(fs) => {
        //         let path = self.get_metadata_path(metadata, supplementary).await?;
        //         let url = fs
        //             .signed_url(Method::GET, &path, Duration::from_secs(500))
        //             .await?;
        //         Ok(SignedUrl {
        //             url: url.to_string(),
        //             headers: vec![],
        //         })
        //     }
        // }
        let url = if supplementary.is_none() {
            format!(
                "{}/files/metadata/download?id={}",
                env::var("BOSCA_URL_PREFIX").unwrap_or("".to_string()),
                metadata.id
            )
        } else {
            format!(
                "{}/files/metadata/download?id={}&supplementary_id={}",
                env::var("BOSCA_URL_PREFIX").unwrap_or("".to_string()),
                metadata.id,
                supplementary.unwrap()
            )
        };
        let token = match datasource.new_token(principal) {
            Ok(token) => token.token,
            Err(e) => {
                return Err(Error::PermissionDenied {
                    path: url,
                    source: Box::new(e),
                })
            }
        };
        Ok(SignedUrl {
            url: datasource.sign_url(&url),
            headers: vec![SignedUrlHeader {
                name: "Authorization".to_string(),
                value: format!("Bearer {token}"),
            }],
        })
    }

    pub async fn get_collection_upload_signed_url(
        &self,
        datastore: &SecurityDataStore,
        principal: &Principal,
        collection: &Collection,
        supplementary: &Uuid,
    ) -> Result<SignedUrl, Error> {
        // match &self.interface.as_ref() {
        //     ObjectStorageInterface::FileSystem(_) => {
        //         let url = if supplementary.is_none() {
        //             format!(
        //                 "{}/files/upload?id={}",
        //                 env::var("BOSCA_URL_PREFIX").unwrap_or("".to_string()),
        //                 metadata.id
        //             )
        //         } else {
        //             format!(
        //                 "{}/files/upload?id={}&supplementary_id={}",
        //                 env::var("BOSCA_URL_PREFIX").unwrap_or("".to_string()),
        //                 metadata.id,
        //                 supplementary.unwrap()
        //             )
        //         };
        //         let token = match datasource.new_token(principal) {
        //             Ok(token) => token.token,
        //             Err(e) => {
        //                 return Err(Error::PermissionDenied {
        //                     path: url,
        //                     source: Box::new(e),
        //                 })
        //             }
        //         };
        //         Ok(SignedUrl {
        //             url,
        //             headers: vec![SignedUrlHeader {
        //                 name: "Authorization".to_string(),
        //                 value: format!("Bearer {}", token),
        //             }],
        //         })
        //     }
        //     ObjectStorageInterface::S3(fs) => {
        //         let path = self.get_metadata_path(metadata, supplementary).await?;
        //         let url = fs
        //             .signed_url(Method::POST, &path, Duration::from_secs(500))
        //             .await?;
        //         Ok(SignedUrl {
        //             url: url.to_string(),
        //             headers: vec![],
        //         })
        //     }
        // }
        let url = format!(
            "{}/files/collection/upload?id={}&supplementary_id={}",
            env::var("BOSCA_URL_PREFIX").unwrap_or("".to_string()),
            collection.id,
            supplementary
        );
        let token = match datastore.new_token(principal) {
            Ok(token) => token.token,
            Err(e) => {
                return Err(Error::PermissionDenied {
                    path: url,
                    source: Box::new(e),
                })
            }
        };
        Ok(SignedUrl {
            url,
            headers: vec![SignedUrlHeader {
                name: "Authorization".to_string(),
                value: format!("Bearer {token}"),
            }],
        })
    }

    pub async fn get_collection_download_signed_url(
        &self,
        datasource: &SecurityDataStore,
        principal: &Principal,
        collection: &Collection,
        supplementary: &Uuid,
    ) -> Result<SignedUrl, Error> {
        // match &self.interface.as_ref() {
        //     ObjectStorageInterface::FileSystem(_) => {
        //         let url = if supplementary.is_none() {
        //             format!(
        //                 "{}/files/download?id={}",
        //                 env::var("BOSCA_URL_PREFIX").unwrap_or("".to_string()),
        //                 metadata.id
        //             )
        //         } else {
        //             format!(
        //                 "{}/files/download?id={}&supplementary_id={}",
        //                 env::var("BOSCA_URL_PREFIX").unwrap_or("".to_string()),
        //                 metadata.id,
        //                 supplementary.unwrap()
        //             )
        //         };
        //         let token = match datasource.new_token(principal) {
        //             Ok(token) => token.token,
        //             Err(e) => {
        //                 return Err(Error::PermissionDenied {
        //                     path: url,
        //                     source: Box::new(e),
        //                 })
        //             }
        //         };
        //         Ok(SignedUrl {
        //             url,
        //             headers: vec![SignedUrlHeader {
        //                 name: "Authorization".to_string(),
        //                 value: format!("Bearer {}", token),
        //             }],
        //         })
        //     }
        //     ObjectStorageInterface::S3(fs) => {
        //         let path = self.get_metadata_path(metadata, supplementary).await?;
        //         let url = fs
        //             .signed_url(Method::GET, &path, Duration::from_secs(500))
        //             .await?;
        //         Ok(SignedUrl {
        //             url: url.to_string(),
        //             headers: vec![],
        //         })
        //     }
        // }
        let url = format!(
            "{}/files/collection/download?id={}&supplementary_id={}",
            env::var("BOSCA_URL_PREFIX").unwrap_or("".to_string()),
            collection.id,
            supplementary
        );
        let token = match datasource.new_token(principal) {
            Ok(token) => token.token,
            Err(e) => {
                return Err(Error::PermissionDenied {
                    path: url,
                    source: Box::new(e),
                })
            }
        };
        Ok(SignedUrl {
            url: datasource.sign_url(&url),
            headers: vec![SignedUrlHeader {
                name: "Authorization".to_string(),
                value: format!("Bearer {token}"),
            }],
        })
    }
}
