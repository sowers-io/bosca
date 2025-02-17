use crate::models::security::principal::Principal;
use crate::security::token::Token;
use argon2::password_hash::rand_core::{OsRng, RngCore};
use chrono::Utc;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};

#[derive(Clone)]
pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
    secret: Vec<u8>,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
            secret: secret.to_vec(),
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

    pub fn new_refresh_token(&self, principal: &Principal) -> Result<String, Error> {
        let mut random_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut random_bytes);

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&random_bytes);
        bytes.extend_from_slice(principal.id.as_bytes());

        let hashed_key = Sha256::digest(&self.keys.secret);
        let key = Key::<Aes256Gcm>::from_slice(&hashed_key);
        let cipher = Aes256Gcm::new(key);

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, bytes.as_ref())
            .expect("Encryption failed");

        let mut token_data = Vec::new();
        token_data.extend_from_slice(nonce);
        token_data.extend_from_slice(&ciphertext);
        Ok(hex::encode(token_data))
    }

    pub fn new_token(&self, principal: &Principal) -> Result<Token, Error> {
        let claims = Claims::new(principal, &self.aud, &self.iss);
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
}
