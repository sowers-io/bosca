use crate::security::token::Token;
use async_graphql::Object;

pub struct TokenObject<'a> {
    token: &'a Token,
}

impl<'a> TokenObject<'a> {
    pub fn new(token: &'a Token) -> Self {
        Self { token }
    }
}

#[Object(name = "Token")]
impl<'a> TokenObject<'a> {
    async fn token(&self) -> String {
        self.token.token.to_string()
    }
}
