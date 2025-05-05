use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use crate::models::bible::book::Book;

#[derive(SimpleObject, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename = "usfm")]
    usfm_: String
}

#[derive(InputObject)]
pub struct ReferenceInput {
    pub usfm: String,
}

impl From<&ReferenceInput> for Reference {
    fn from(reference: &ReferenceInput) -> Self {
        Self {
            usfm_: reference.usfm.clone(),
        }
    }
}

impl Reference {

    pub fn new(usfm: String) -> Self {
        Self {
            usfm_: usfm,
        }
    }

    pub fn is_usfm(&self, usfm: &str) -> bool {
        self.usfm_.eq_ignore_ascii_case(usfm)
    }

    pub fn usfm(&self) -> &String {
        &self.usfm_
    }

    pub fn format(&self, book: &Book) -> String {
        let mut human = book.name_long.clone();
        if let Some(chapter) = self.chapter() {
            human.push(' ');
            human.push_str(&chapter);
            let mut verses = Vec::new();
            let references = self.references();
            for reference in references {
                if let Some(verse) = reference.verse() {
                    if let Ok(verse) = verse.parse::<i32>() {
                        verses.push(verse);
                    }
                }
            }
            if !verses.is_empty() {
                verses.sort();
                human.push(':');
                if verses.len() > 1 {
                    human.push_str(verses.first().unwrap().to_string().as_str());
                    human.push('-');
                    human.push_str(verses.last().unwrap().to_string().as_str());
                } else {
                    human.push_str(verses.first().unwrap().to_string().as_str());
                }
            }
        }
        human
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

    pub fn chapter(&self) -> Option<String> {
        self.chapter_usfm().map(|s| s.split('.').nth(1).unwrap_or_default().to_string())
    }

    pub fn verse(&self) -> Option<String> {
        self.verse_usfm().map(|s| s.split('.').nth(2).unwrap_or_default().to_string())
    }

    pub fn references(&self) -> Vec<Reference> {
        self.usfm_
            .split('+')
            .map(|r| Reference::new(r.to_string()))
            .collect()
    }
}