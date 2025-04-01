use crate::models::bible::components::style::{Style, StyleInput};
use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Break {
    pub style: Option<Style>
}

#[derive(InputObject)]
pub struct BreakInput {
    pub style: Option<StyleInput>
}

impl From<&BreakInput> for Break {
    fn from(b: &BreakInput) -> Self {
        Self {
            style: b.style.as_ref().map(|s| s.into()),
        }
    }
}