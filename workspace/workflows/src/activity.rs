use std::env::VarError;
use async_trait::async_trait;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::Utf8Error;
use duckdb::arrow::error::ArrowError;
use tokio::fs::{remove_file, File};
use tokio::io::AsyncWriteExt;
use tokio::task::JoinError;
use uuid::Uuid;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::add_activity::ActivityInput;

pub struct ActivityContext {
    files_to_clean: Vec<String>,
}

impl Default for ActivityContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ActivityContext {
    pub fn new() -> Self {
        Self { files_to_clean: Vec::new() }
    }
    pub fn add_file_clean(&mut self, name: &str) {
        self.files_to_clean.push(name.to_owned());
    }

    pub async fn write_to_file(&mut self, content: &[u8]) -> Result<String, Error> {
        let name = format!("/tmp/bosca/{}", Uuid::new_v4());
        let mut file = File::create_new(&name).await.map_err(|e| Error::new(format!("error creating file: {e:?}")))?;
        file.write_all(content)
            .await
            .map_err(|e| Error::new(format!("error writing to file: {e:?}")))?;
        self.add_file_clean(&name);
        Ok(name)
    }

    pub async fn new_file(&mut self, ext: &str) -> Result<String, Error> {
        let name = if !ext.is_empty() {
            format!("/tmp/bosca/{}.{}", Uuid::new_v4(), ext)
        } else {
            format!("/tmp/bosca/{}", Uuid::new_v4())
        };
        self.add_file_clean(&name);
        Ok(name)
    }

    pub async fn close(&mut self) -> Result<(), Error> {
        for file in self.files_to_clean.iter() {
            let _ = remove_file(Path::new(file)).await;
        }
        Ok(())
    }
}

#[async_trait]
pub trait Activity {
    fn id(&self) -> &String;

    fn create_activity_input(&self) -> ActivityInput;

    async fn execute(&self, client: &Client, context: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<JoinError> for Error {
    fn from(value: JoinError) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<duckdb::Error> for Error {
    fn from(value: duckdb::Error) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<ArrowError> for Error {
    fn from(value: ArrowError) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<VarError> for Error {
    fn from(value: VarError) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<graphql_client::Error> for Error {
    fn from(value: graphql_client::Error) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<bosca_client::Error> for Error {
    fn from(value: bosca_client::Error) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}
