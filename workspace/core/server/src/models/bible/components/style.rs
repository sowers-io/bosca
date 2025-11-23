use async_graphql::{InputObject, SimpleObject, Union};
use serde::{Deserialize, Serialize};

#[derive(Union, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Style {
    #[serde(rename = "s")]
    Declared(DeclaredStyle),
    #[serde(rename = "sr")]
    Referenced(StyleReference),
}

#[derive(Union, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "_")]
pub enum Style2 {
    #[serde(rename = "s")]
    Declared(DeclaredStyle),
    #[serde(rename = "sr")]
    Referenced(StyleReference),
}

#[derive(SimpleObject, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeclaredStyle {
    pub id: String,
    pub align: Option<String>,
    pub text_indent: Option<TextIndent>,
    pub font_weight: Option<String>,
}

#[derive(SimpleObject, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleReference {
    pub id: String,
}

#[derive(SimpleObject, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextIndent {
    pub size: f64,
    pub unit: String,
}

#[derive(InputObject)]
pub struct TextIndentInput {
    pub size: f64,
    pub unit: String,
}

impl From<&TextIndentInput> for TextIndent {
    fn from(value: &TextIndentInput) -> Self {
        Self {
            size: value.size,
            unit: value.unit.clone(),
        }
    }
}

#[derive(InputObject)]
pub struct StyleInput {
    pub reference: bool,
    pub id: String,
    pub align: Option<String>,
    pub text_indent: Option<TextIndentInput>,
    pub font_weight: Option<String>,
}

impl From<&StyleInput> for Style {
    fn from(value: &StyleInput) -> Self {
        if value.reference {
            Style::Referenced(StyleReference {
                id: value.id.clone()
            })
        } else {
            Style::Declared(DeclaredStyle {
                id: value.id.to_string(),
                align: value.align.clone(),
                text_indent: value.text_indent.as_ref().map(|t| t.into()),
                font_weight: value.font_weight.clone(),
            })
        }
    }
}