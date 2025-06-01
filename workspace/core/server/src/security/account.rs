use crate::models::profiles::profile::ProfileInput;
use crate::models::profiles::profile_attribute::ProfileAttributeInput;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use async_graphql::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Email {
    email: String,
}

#[derive(Serialize, Deserialize)]
struct Name {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    google: Option<GoogleAccount>,
    facebook: Option<FacebookUser>,
}

impl Account {
    pub fn new_google(account: GoogleAccount) -> Self {
        Self {
            google: Some(account),
            facebook: None,
        }
    }

    pub fn new_facebook(account: FacebookUser) -> Self {
        Self {
            google: None,
            facebook: Some(account),
        }
    }

    pub fn id(&self) -> Option<&str> {
        if let Some(google) = &self.google {
            Some(&google.sub)
        } else if let Some(facebook) = &self.facebook {
            Some(&facebook.id)
        } else {
            None
        }
    }

    pub fn verified(&self) -> bool {
        if let Some(google) = &self.google {
            google.email_verified
        } else if let Some(facebook) = &self.facebook {
            !facebook.email.is_empty()
        } else {
            false
        }
    }

    pub fn oauth2_type(&self) -> String {
        if self.google.is_some() {
            "google".to_string()
        } else if self.facebook.is_some() {
            "facebook".to_string()
        } else {
            "".to_string()
        }
    }

    pub fn new_profile(&self) -> Result<Option<ProfileInput>, Error> {
        if let Some(google) = &self.google {
            let email = Email {
                email: google.email.clone(),
            };
            let name = Name {
                name: google.name.clone(),
            };
            Ok(Some(ProfileInput {
                slug: None,
                name: google.name.to_string(),
                visibility: ProfileVisibility::User,
                attributes: vec![
                    ProfileAttributeInput {
                        id: None,
                        type_id: "bosca.profiles.email".to_string(),
                        visibility: ProfileVisibility::User,
                        confidence: 100,
                        priority: 1,
                        source: "google".to_string(),
                        attributes: Some(serde_json::to_value(&email)?),
                        metadata_id: None,
                        metadata_supplementary: None,
                        expiration: None,
                    },
                    ProfileAttributeInput {
                        id: None,
                        type_id: "bosca.profiles.name".to_string(),
                        visibility: ProfileVisibility::User,
                        confidence: 100,
                        priority: 1,
                        source: "google".to_string(),
                        attributes: Some(serde_json::to_value(&name)?),
                        metadata_id: None,
                        metadata_supplementary: None,
                        expiration: None,
                    },
                ],
            }))
        } else if let Some(facebook) = &self.facebook {
            let email = Email {
                email: facebook.email.clone(),
            };
            let name = Name {
                name: facebook.name.clone(),
            };
            Ok(Some(ProfileInput {
                slug: None,
                name: name.name.clone(),
                visibility: ProfileVisibility::User,
                attributes: vec![
                    ProfileAttributeInput {
                        id: None,
                        type_id: "bosca.profiles.email".to_string(),
                        visibility: ProfileVisibility::User,
                        confidence: 100,
                        priority: 1,
                        source: "facebook".to_string(),
                        attributes: Some(serde_json::to_value(&email)?),
                        metadata_id: None,
                        metadata_supplementary: None,
                        expiration: None,
                    },
                    ProfileAttributeInput {
                        id: None,
                        type_id: "bosca.profiles.name".to_string(),
                        visibility: ProfileVisibility::User,
                        confidence: 100,
                        priority: 1,
                        source: "facebook".to_string(),
                        attributes: Some(serde_json::to_value(&name)?),
                        metadata_id: None,
                        metadata_supplementary: None,
                        expiration: None,
                    },
                ],
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct GoogleAccount {
    #[serde(default)]
    pub sub: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub given_name: String,
    #[serde(default)]
    pub family_name: String,
    #[serde(default)]
    pub picture: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub email_verified: bool,
    #[serde(default)]
    pub hd: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct FacebookUser {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub picture: FacebookPicture,
}

#[derive(Default, Serialize, Deserialize)]
pub struct FacebookPicture {
    #[serde(default)]
    pub data: FacebookPictureData,
}

#[derive(Default, Serialize, Deserialize)]
pub struct FacebookPictureData {
    #[serde(default)]
    pub height: i64,
    #[serde(default)]
    pub is_silhouette: bool,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub width: i64,
}
