use crate::models::bible::components::break_component::{Break, BreakInput};
use crate::models::bible::components::container::{ComponentContainer, ComponentContainerInput};
use crate::models::bible::components::text::{Text, TextInput};
use crate::models::bible::components::verse_end::{VerseEnd, VerseEndInput};
use crate::models::bible::components::verse_start::{VerseStart, VerseStartInput};
use async_graphql::{InputObject, Union};
use serde::{Deserialize, Serialize};
use crate::models::bible::reference::Reference;

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

struct FilterContext {
    found_starts: i32,
}

impl Component {
    pub fn filter(&self, reference: &Reference) -> Option<Component> {
        let mut ctx = FilterContext { found_starts: 0 };
        let refs = reference.references();
        self.filter_inner(&refs, &mut ctx)
    }

    fn filter_inner(&self, reference: &Vec<Reference>, ctx: &mut FilterContext) -> Option<Component> {
        match self {
            Component::VerseStart(vs) => {
                if reference.iter().any(|r| r.is_usfm(vs.reference.usfm())) {
                    ctx.found_starts += 1;
                    Some(Component::VerseStart(vs.clone()))
                } else {
                    None
                }
            }
            Component::VerseEnd(end) => {
                if ctx.found_starts > 0 {
                    ctx.found_starts -= 1;
                    Some(Component::VerseEnd(end.clone()))
                } else {
                    None
                }
            }
            Component::Break(b) => {
                if ctx.found_starts > 0 {
                    Some(Component::Break(b.clone()))
                } else {
                    None
                }
            }
            Component::Text(txt) => {
                if ctx.found_starts > 0 {
                    Some(Component::Text(txt.clone()))
                } else {
                    None
                }
            }
            Component::Container(c) => {
                let filtered: Vec<Component> = c.components.iter().filter_map(|c| c.filter_inner(reference, ctx)).collect();
                if filtered.is_empty() {
                    None
                } else {
                    Some(Component::Container(ComponentContainer {
                        components: filtered,
                        ..c.clone()
                    }))
                }
            }
        }
    }
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