use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub usfm: String,
}

#[derive(InputObject)]
pub struct ReferenceInput {
    pub usfm: String,
}

impl From<&ReferenceInput> for Reference {
    fn from(reference: &ReferenceInput) -> Self {
        Self {
            usfm: reference.usfm.clone(),
        }
    }
}