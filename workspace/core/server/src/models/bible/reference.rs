use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename = "usfm")]
    usfm_: String,
    #[serde(skip)]
    references_: Option<Vec<Reference>>,
}

#[derive(InputObject)]
pub struct ReferenceInput {
    pub usfm: String,
}

impl From<&ReferenceInput> for Reference {
    fn from(reference: &ReferenceInput) -> Self {
        Self {
            usfm_: reference.usfm.clone(),
            references_: None,
        }
    }
}

impl Reference {

    pub fn new(usfm: String) -> Self {
        Self {
            usfm_: usfm,
            references_: None,
        }
    }

    pub fn is_usfm(&self, usfm: &str) -> bool {
        self.usfm_.eq_ignore_ascii_case(usfm)
    }

    pub fn usfm(&self) -> &String {
        &self.usfm_
    }

    pub fn book_usfm(&self) -> Option<String> {
        self.usfm_.split('.').next().map(|s| s.to_string())
    }

    pub fn chapter_usfm(&self) -> Option<String> {
        let parts: Vec<&str> = self.usfm_.split('.').collect();
        if parts.len() >= 2 {
            Some(format!("{}.{}", parts[0], parts[1]))
        } else {
            None
        }
    }

    pub fn verse_usfm(&self) -> Option<String> {
        let parts: Vec<&str> = self.usfm_.split('.').collect();
        if parts.len() >= 3 {
            Some(format!("{}.{}.{}", parts[0], parts[1], parts[2]))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn chapter(&self) -> String {
        self.usfm_.split('.').nth(1).unwrap_or_default().to_string()
    }

    pub fn number(&self) -> String {
        self.usfm_.split('.').last().unwrap_or_default().to_string()
    }

    pub fn references(&self) -> Vec<Reference> {
        self.usfm_
            .split('+')
            .map(|r| Reference::new(r.to_string()))
            .collect()
    }
}