use crate::models::bible::reference::{Reference, ReferenceInput};
use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerseEnd {
    pub reference: Reference,
}

#[derive(InputObject)]
pub struct VerseEndInput {
    pub reference: ReferenceInput,
}

impl From<&VerseEndInput> for VerseEnd {
    fn from(end: &VerseEndInput) -> Self {
        Self {
            reference: (&end.reference).into(),
        }
    }
}