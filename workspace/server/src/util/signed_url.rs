use std::str::FromStr;
use hmac::{Hmac, Mac};
use http::Uri;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

// Function to sign a URL and include an expiration timestamp
pub fn sign_url(url: &str, secret_key: &str, duration_secs: u64) -> String {
    let uri = Uri::from_str(url).expect("Invalid URL");

    // Get the current UNIX timestamp
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Calculate expiration time
    let expiration = current_time + duration_secs;

    let path = uri.path_and_query().unwrap().as_str();

    // Include the `expires` parameter
    let unsigned_path = if path.contains('?') {
        format!("{path}&expires={expiration}")
    } else {
        format!("{path}?expires={expiration}")
    };

    // Create HMAC-SHA256 signer
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
        .expect("HMAC can take a key of any size");
    mac.update(unsigned_path.as_bytes());

    // Generate cryptographic signature
    let signature = hex::encode(mac.finalize().into_bytes());

    let unsigned_url = if path.contains('?') {
        format!("{url}&expires={expiration}")
    } else {
        format!("{url}?expires={expiration}")
    };

    // Append signature to the URL
    format!("{unsigned_url}&signature={signature}")
}

// Function to verify a signed URL
pub fn verify_signed_url(url: &str, secret_key: &str) -> bool {
    let url = Uri::from_str(url).expect("Invalid URL");
    let query_params: Vec<(String, String)> = url
        .path_and_query()
        .map(|query| {
            let query_string = query.query().unwrap_or_default();
            let query_parts: Vec<&str> = query_string.split('&').collect();
            let mut parts = Vec::new();
            for part in query_parts {
                let kv: Vec<&str> = part.split('=').collect();
                if kv.len() == 2 {
                    parts.push((kv[0].to_string(), kv[1].to_string()));
                } else if kv.len() == 1 {
                    parts.push((kv[0].to_string(), "".to_string()));
                }
            }
            parts
        }).unwrap();

    let mut expires = None;
    let mut signature = None;

    let mut unsigned_url = url.path().to_string();
    let mut first = true;

    // Extract and rebuild the URL query string (excluding the signature parameter)
    for (key, value) in query_params {
        if key == "signature" {
            signature = Some(value);
            continue;
        }
        if key == "expires" {
            expires = Some(value.parse::<u64>().expect("Invalid expires value"));
        }
        if first {
            unsigned_url.push('?');
            first = false;
        } else {
            unsigned_url.push('&');
        }
        unsigned_url.push_str(&format!("{key}={value}"));
    }

    // Check that the URL has an expiration time
    let expires = match expires {
        Some(e) => e,
        None => return false, // No expiration time found
    };

    // Check if the URL has expired
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    if expires < current_time {
        return false; // URL is expired
    }

    // Recreate the signature
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).expect("Invalid HMAC key");
    mac.update(unsigned_url.as_bytes());
    let calculated_signature = hex::encode(mac.finalize().into_bytes());

    // Compare the provided signature with the calculated signature
    match signature {
        Some(sig) => sig == calculated_signature,
        None => false, // No signature provided
    }
}
