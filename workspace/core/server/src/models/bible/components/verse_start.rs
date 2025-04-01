use crate::models::bible::reference::{Reference, ReferenceInput};
use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerseStart {
    pub reference: Reference,
}

#[derive(InputObject)]
pub struct VerseStartInput {
    pub reference: ReferenceInput,
}

impl From<&VerseStartInput> for VerseStart {
    fn from(value: &VerseStartInput) -> Self {
        Self {
            reference: (&value.reference).into(),
        }
    }
}
