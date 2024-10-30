use std::fmt::{Display, Formatter};

pub mod client;
pub mod download;
pub mod upload;

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

