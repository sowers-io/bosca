use crate::security::jwt::{Claims, Keys};
use async_graphql::SimpleObject;
use jsonwebtoken::errors::Error;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
    pub issued_at: usize,
    pub expires_at: usize,
}

impl Token {
    pub fn new(claims: &Claims, key: &Keys) -> Result<Token, Error> {
        Ok(Self {
            token: key.encode(claims)?,
            issued_at: claims.iat,
            expires_at: claims.exp,
        })
    }
}
