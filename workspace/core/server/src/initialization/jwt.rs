use std::env;
use std::process::exit;
use crate::security::jwt::{Jwt, Keys};

pub fn new_jwt() -> Jwt {
    let keys = match env::var("JWT_SECRET") {
        Ok(secret) => Keys::new(secret.as_bytes()),
        _ => {
            println!("Environment variable JWT_SECRET could not be read");
            exit(1);
        }
    };
    let audience = match env::var("JWT_AUDIENCE") {
        Ok(audience) => audience,
        _ => {
            println!("Environment variable JWT_SECRET could not be read");
            exit(1);
        }
    };
    let issuer = match env::var("JWT_ISSUER") {
        Ok(issuer) => issuer,
        _ => {
            println!("Environment variable JWT_SECRET could not be read");
            exit(1);
        }
    };
    Jwt::new(keys, &audience, &issuer)
}