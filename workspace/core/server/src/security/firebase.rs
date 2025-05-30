use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct HashConfig {
    pub algorithm: String,
    pub base64_signer_key: String,
    pub base64_salt_separator: String,
    pub rounds: u32,
    pub mem_cost: u32,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FirebaseImportProviderInfo {
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "rawId")]
    pub raw_id: String,
    #[serde(default)]
    pub email: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "photoUrl")]
    pub photo_url: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FirebaseImportUser {
    #[serde(rename = "localId")]
    pub local_id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default, rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(default)]
    pub salt: String,
    #[serde(default, rename = "passwordHash")]
    pub password_hash: String,
    #[serde(default, rename = "displayName")]
    pub display_name: String,
    #[serde(default, rename = "photoUrl")]
    pub photo_url: String,
    #[serde(default, rename = "lastSignedInAt")]
    pub last_signed_in_at: String,
    #[serde(default, rename = "createdAt")]
    pub created_at: String,
    #[serde(default, rename = "providerUserInfo")]
    pub provider_user_info: Vec<FirebaseImportProviderInfo>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FirebaseImportUsers {
    pub users: Vec<FirebaseImportUser>,
}