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
}

impl Account {
    pub fn new_google(account: GoogleAccount) -> Self {
        Self {
            google: Some(account),
        }
    }

    pub fn id(&self) -> Option<&str> {
        if let Some(google) = &self.google {
            Some(&google.sub)
        } else {
            None
        }
    }

    pub fn verified(&self) -> bool {
        if let Some(google) = &self.google {
            google.email_verified
        } else {
            false
        }
    }

    pub fn oauth2_type(&self) -> String {
        if let Some(_) = &self.google {
            "google".to_string()
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
