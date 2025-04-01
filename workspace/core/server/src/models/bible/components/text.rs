use crate::models::bible::components::style::{Style, StyleInput};
use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Text {
    pub text: String,
    pub style: Option<Style>,
}

#[derive(InputObject)]
pub struct TextInput {
    pub text: String,
    pub style: Option<StyleInput>,
}

impl From<&TextInput> for Text {
    fn from(text: &TextInput) -> Self {
        Self {
            style: text.style.as_ref().map(|s| s.into()),
            text: text.text.clone(),
        }
    }
}