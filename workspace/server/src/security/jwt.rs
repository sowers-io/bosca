use crate::models::security::principal::Principal;
use crate::security::token::Token;
use chrono::Utc;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }

    pub fn encode(&self, claims: &Claims) -> Result<String, Error> {
        encode(&Header::default(), claims, &self.encoding)
    }
}

#[derive(Clone)]
pub struct Jwt {
    keys: Keys,
    aud: String,
    iss: String,
    validation: Validation,
}

impl Jwt {
    pub fn new(keys: Keys, audience: &str, issuer: &str) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[audience]);
        Self {
            keys,
            aud: audience.to_owned(),
            iss: issuer.to_owned(),
            validation,
        }
    }

    pub fn new_token(&self, principal: &Principal) -> Result<Token, Error> {
        let claims = Claims::new(principal, &self.aud, &self.iss);
        Token::new(&claims, &self.keys)
    }

    pub fn new_verification_token(&self) -> Result<Token, Error> {
        let claims = Claims::new_verification(&self.aud, &self.iss);
        Token::new(&claims, &self.keys)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, Error> {
        let token = decode::<Claims>(token, &self.keys.decoding, &self.validation)?;
        Ok(token.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub sub: String,
}

impl Claims {
    pub fn new(principal: &Principal, audience: &str, issuer: &str) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            sub: principal.id.to_string(),
            exp: (now + chrono::naive::Days::new(1)).and_utc().timestamp() as usize,
            iat: now.and_utc().timestamp() as usize,
            iss: issuer.to_owned(),
            aud: audience.to_owned(),
        }
    }

    pub fn new_verification(audience: &str, issuer: &str) -> Self {
        let now = Utc::now().naive_utc();
        let uuid = Uuid::new_v4().to_string();
        Self {
            sub: format!("verification:{}", uuid),
            exp: (now + chrono::naive::Days::new(1)).and_utc().timestamp() as usize,
            iat: now.and_utc().timestamp() as usize,
            iss: issuer.to_owned(),
            aud: audience.to_owned(),
        }
    }
}
