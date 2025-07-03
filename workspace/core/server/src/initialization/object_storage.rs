use crate::graphql::content::storage::{ObjectStorage, ObjectStorageInterface};
use log::info;
use object_store::aws::AmazonS3Builder;
use object_store::local::LocalFileSystem;
use std::env;
use std::fs::create_dir_all;
use std::path::Path;
use std::sync::Arc;
use object_store::gcp::GoogleCloudStorageBuilder;
use object_store::ObjectStoreScheme::GoogleCloudStorage;

fn new_filesystem_object_storage() -> ObjectStorage {
    let current_dir = match env::var("STORAGE") {
        Ok(dir) => Path::new(dir.as_str()).to_path_buf(),
        _ => env::current_dir().unwrap().join(Path::new("files")),
    };
    let path = current_dir.as_path();
    if !path.exists() {
        create_dir_all(path).unwrap();
    }
    info!("Using file object storage at path: {path:?}");
    ObjectStorage::new(ObjectStorageInterface::FileSystem(Arc::new(
        LocalFileSystem::new_with_prefix(path).unwrap(),
    )))
}

fn new_s3_object_storage() -> ObjectStorage {
    info!("Using s3 object storage");
    ObjectStorage::new(ObjectStorageInterface::S3(Arc::new(
        AmazonS3Builder::from_env().build().unwrap(),
    )))
}

fn new_gcp_object_storage() -> ObjectStorage {
    info!("Using gcp object storage");
    ObjectStorage::new(ObjectStorageInterface::GCP(Arc::new(
        GoogleCloudStorageBuilder::from_env().build().unwrap(),
    )))
}

pub fn new_object_storage() -> ObjectStorage {
    match env::var("STORAGE") {
        Ok(name) => match name.as_str() {
            "s3" => new_s3_object_storage(),
            "gcp" => new_gcp_object_storage(),
            _ => new_filesystem_object_storage(),
        },
        _ => new_filesystem_object_storage(),
    }
}
