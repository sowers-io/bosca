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
impl TokenObject<'_> {
    async fn token(&self) -> String {
        self.token.token.to_string()
    }

    async fn issued_at(&self) -> usize { self.token.issued_at }

    async fn expires_at(&self) -> usize { self.token.expires_at }
}
