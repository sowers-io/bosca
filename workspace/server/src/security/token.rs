use crate::security::jwt::{Claims, Keys};
use async_graphql::Object;
use jsonwebtoken::errors::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

impl Token {
    pub fn new(claims: &Claims, key: &Keys) -> Result<Token, Error> {
        Ok(Self {
            token: key.encode(claims)?,
        })
    }
}

#[Object]
impl Token {
    async fn token(&self) -> &String {
        &self.token
    }
}
