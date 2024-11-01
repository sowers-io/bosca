use std::env::VarError;
use async_trait::async_trait;
use std::fmt::{Display, Formatter};
use std::path::Path;
use tokio::fs::remove_file;
use bosca_client::client::{Client, WorkflowJob};

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
    pub fn add_file_clean(&mut self, name: String) {
        self.files_to_clean.push(name);
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
