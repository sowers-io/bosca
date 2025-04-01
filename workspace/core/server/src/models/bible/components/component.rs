use crate::models::bible::components::break_component::{Break, BreakInput};
use crate::models::bible::components::container::{ComponentContainer, ComponentContainerInput};
use crate::models::bible::components::text::{Text, TextInput};
use crate::models::bible::components::verse_end::{VerseEnd, VerseEndInput};
use crate::models::bible::components::verse_start::{VerseStart, VerseStartInput};
use async_graphql::{InputObject, Union};
use serde::{Deserialize, Serialize};

#[derive(Union, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "_")]
pub enum Component {
    #[serde(rename = "vs")]
    VerseStart(VerseStart),
    #[serde(rename = "ve")]
    VerseEnd(VerseEnd),
    #[serde(rename = "b")]
    Break(Break),
    #[serde(rename = "t")]
    Text(Text),
    #[serde(rename = "cc")]
    Container(ComponentContainer),
}

#[derive(InputObject)]
pub struct ComponentInput {
    pub start: Option<VerseStartInput>,
    pub end: Option<VerseEndInput>,
    #[graphql(name = "break")]
    pub break_component: Option<BreakInput>,
    pub text: Option<TextInput>,
    pub container: Option<ComponentContainerInput>,
}

impl From<&ComponentInput> for Component {
    fn from(component: &ComponentInput) -> Self {
        if let Some(start) = component.start.as_ref() {
            Component::VerseStart(start.into())
        } else if let Some(end) = component.end.as_ref() {
            Component::VerseEnd(end.into())
        } else if let Some(break_component) = component.break_component.as_ref() {
            Component::Break(break_component.into())
        } else if let Some(text) = component.text.as_ref() {
            Component::Text(text.into())
        } else if let Some(container) = component.container.as_ref() {
            Component::Container(container.into())
        } else {
            panic!("Invalid component input")
        }
    }
}