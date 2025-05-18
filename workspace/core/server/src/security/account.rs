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

#[derive(Serialize, Deserialize)]
pub struct GoogleAccount {
    pub sub: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub email: String,
    pub email_verified: bool,
    pub hd: String,
}

#[derive(Serialize, Deserialize)]
pub struct FacebookUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub picture: FacebookPicture,
}

#[derive(Serialize, Deserialize)]
pub struct FacebookPicture {
    pub data: FacebookPictureData,
}

#[derive(Serialize, Deserialize)]
pub struct FacebookPictureData {
    pub height: i64,
    pub is_silhouette: bool,
    pub url: String,
    pub width: i64,
}
