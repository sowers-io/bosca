use crate::cache::cache_configuration::CacheConfiguration;
use std::error::Error;
use std::path::PathBuf;
use std::fs;
use tokio::fs as tokio_fs;

#[derive(Clone)]
pub struct Store {
    path: PathBuf,
}

impl Store {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub async fn load(&self) -> Result<Vec<CacheConfiguration>, Box<dyn Error>> {
        if !self.path.exists() {
            fs::create_dir_all(&self.path)?;
            return Ok(vec![]);
        }

        let mut entries = tokio_fs::read_dir(&self.path).await?;
        let mut configurations = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            if let Some(extension) = entry.path().extension() {
                if extension == "json" {
                    let content = tokio_fs::read_to_string(entry.path()).await?;
                    let config: CacheConfiguration = serde_json::from_str(&content)?;
                    configurations.push(config);
                }
            }
        }

        Ok(configurations)
    }

    pub async fn save(&self, configuration: CacheConfiguration) -> Result<(), Box<dyn Error>> {
        if !self.path.exists() {
            fs::create_dir_all(&self.path)?;
        }

        let file_path = self.path.join(format!("{}.json", configuration.id));
        let content = serde_json::to_string_pretty(&configuration)?;
        tokio_fs::write(file_path, content).await?;

        Ok(())
    }

    // pub async fn delete(&self, id: &str) {
    //     let file_path = self.path.join(format!("{}.json", id));
    //
    //     if let Ok(metadata) = tokio_fs::metadata(&file_path).await {
    //         if metadata.is_file() {
    //             if let Err(e) = tokio_fs::remove_file(file_path).await {
    //                 eprintln!("Failed to delete cache configuration: {}", e);
    //             }
    //         }
    //     }
    // }
}