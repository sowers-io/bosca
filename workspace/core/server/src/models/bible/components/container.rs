use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use crate::models::bible::components::component::{Component, ComponentInput};
use crate::models::bible::components::style::{Style, StyleInput};

#[derive(Enum, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Copy, Ord, PartialOrd)]
pub enum ContainerType {
    #[serde(rename = "DIV")]
    Div,
    #[serde(rename = "SPAN")]
    Span,
    #[serde(rename = "PARAGRAPH")]
    Paragraph,
    #[serde(rename = "TABLE")]
    Table,
    #[serde(rename = "ROW")]
    Row,
    #[serde(rename = "COLUMN")]
    Column,
}

#[derive(SimpleObject, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentContainer {
    #[serde(rename = "type")]
    #[graphql(name = "type")]
    pub container_type: ContainerType,
    pub components: Vec<Component>,
    pub style: Option<Style>
}

#[derive(InputObject)]
pub struct ComponentContainerInput {
    #[graphql(name = "type")]
    pub container_type: ContainerType,
    pub components: Vec<ComponentInput>,
    pub style: Option<StyleInput>
}

impl From<&ComponentContainerInput> for ComponentContainer {
    fn from(value: &ComponentContainerInput) -> Self {
        Self {
            container_type: value.container_type,
            components: value.components.iter().map(|c| c.into()).collect(),
            style: value.style.as_ref().map(|s| s.into())
        }
    }
}